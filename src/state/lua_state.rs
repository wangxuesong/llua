use crate::api::constants::*;
use crate::api::{luaState, lua_State};
use crate::chunk::binary::{Constant, ConstantValue, Prototype};
use crate::state::{LuaClosure, LuaStack, LuaTable, LuaValue};
use crate::vm::Instruction;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CallInfo {
    func: Rc<RefCell<LuaClosure>>,
    pc: usize,
    pub base: isize,
    pub top: isize,
    pub nresults: isize,
}

impl CallInfo {
    pub fn new(proto: Rc<RefCell<LuaClosure>>, base: isize) -> Self {
        let top = base + proto.borrow().proto.max_stack_size as isize;
        CallInfo {
            func: proto,
            pc: 0,
            base,
            top,
            nresults: 0,
        }
    }

    pub fn fetch(&mut self) -> Option<u32> {
        if self.pc < self.func.borrow().proto.code.len() {
            let inst = self.func.borrow().proto.code[self.pc];
            self.pc += 1;
            Some(inst)
        } else {
            None
        }
    }

    pub fn get_const(&self, index: isize) -> Constant {
        self.func.borrow().proto.constants[index as usize].clone()
    }

    pub fn load_proto(&self, index: isize) -> Rc<Prototype> {
        self.func.borrow().proto.prototypes[index as usize].clone()
    }

    pub fn get_base(&self) -> isize {
        self.base.clone()
    }

    pub fn get_top(&self) -> isize {
        self.top.clone()
    }
}

#[derive(Clone)]
pub struct LuaState {
    registry: LuaValue,
    pub stack: LuaStack,
    base: isize,
    base_ci: Vec<Rc<RefCell<CallInfo>>>,
    ci: isize,
}

impl LuaState {
    pub fn new() -> LuaState {
        let registry = LuaValue::new_table(3, 0);
        if let LuaValue::Table(t) = &registry {
            let global = LuaValue::new_table(0, 0);
            t.borrow_mut().set_array(LUA_RIDX_GLOBALS, global);
        }
        LuaState {
            registry,
            stack: LuaStack::new(30),
            base: 0,
            base_ci: vec![],
            ci: -1,
        }
    }

    pub fn set_top(&mut self, index: &isize) {
        self.stack.set_top(index)
    }

    pub fn fetch(&mut self) -> Option<u32> {
        if self.base_ci.len() == 0 {
            return None;
        }
        self.base_ci[self.ci as usize].borrow_mut().fetch()
    }

    pub fn get_const(&mut self, index: isize) -> LuaValue {
        let c = self.base_ci[self.ci as usize]
            .borrow()
            .get_const(index)
            .clone();
        let v = match &c.const_value {
            ConstantValue::Nil => LuaValue::Nil,
            ConstantValue::Integer(v) => LuaValue::Integer(*v),
            ConstantValue::ShortStr(v) => LuaValue::String(v.clone().value),
            _ => {
                dbg!(&c.const_value);
                unimplemented!()
            }
        };
        v
    }

    pub fn get_rk(&mut self, index: isize) -> LuaValue {
        if index > 0xFF {
            self.get_const(index - 0xFF - 1)
        } else {
            let v = self.stack.stack[(self.base + index) as usize].clone();
            v
        }
    }

    pub fn get_value(&self, index: isize) -> LuaValue {
        self.stack.get(self.base + index)
    }

    pub fn load_proto(&self, proto: Rc<Prototype>) -> LuaValue {
        let mut closure = LuaClosure::new(proto.clone());
        let n = proto.upvalues.len();
        if n > 0 {
            for i in 0..n {
                if proto.upvalues[i].instack == 1 {
                    let v = if proto.upvalue_names[i].value.eq("_ENV") {
                        if let LuaValue::Table(t) = &self.registry {
                            let global = t
                                .borrow_mut()
                                .get(LuaValue::Integer(LUA_RIDX_GLOBALS as i64));
                            global
                        } else {
                            LuaValue::Nil
                        }
                    } else {
                        self.get_value(proto.upvalues[i].idx.clone() as isize)
                    };
                    closure.upvalues.push(v);
                }
            }
        }

        LuaValue::Closure(Rc::new(RefCell::new(closure)))
    }

    pub fn get_subproto(&self, index: isize) -> Rc<Prototype> {
        self.base_ci[self.ci as usize]
            .borrow()
            .load_proto(index)
            .clone()
    }

    pub fn create_table(&mut self, array_size: isize, hash_size: isize) -> LuaValue {
        LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(
            array_size as usize,
            hash_size as usize,
        ))))
    }

    pub fn set_value(&mut self, index: isize, value: LuaValue) {
        self.stack.set(self.base + index, value);
    }

    pub fn get_upvalue(&self, index: isize) -> LuaValue {
        let ci = self.base_ci[self.ci as usize].clone();
        let x = ci.borrow().func.borrow().upvalues[index as usize].clone();
        x
    }

    pub fn precall(&mut self, a: isize, b: isize, _c: isize) {
        if let LuaValue::Closure(func) = self.get_value(a) {
            let s = self.clone();
            if func.borrow().function.is_some() {
                func.borrow().function.unwrap()(Rc::new(RefCell::new(s)));
                let argc = b - 1;
                for _ in 0..argc {
                    self.stack.pop();
                }
                return;
            }
            let mut ci = CallInfo::new(func.clone(), self.base + a + 1);
            ci.nresults = _c - 1;
            self.base = ci.get_base();
            let top = ci.get_top();
            self.set_top(&top);
            self.base_ci.push(Rc::new(RefCell::new(ci)));
            self.ci += 1;
        }
    }

    pub fn postcall(&mut self, a: isize, b: isize, _c: isize) {
        if b == 1 {
            return;
        }

        let _ci = self.base_ci.pop().unwrap();
        self.ci -= 1;

        let mut res = _ci.borrow().base - 1;
        let wanted = _ci.borrow().nresults;

        if b > 1 {
            let mut index = res;
            for i in a..b - 1 + a {
                self.set_value(index - self.base, self.get_value(i));
                index += 1;
            }
            res = index;
        }

        if self.ci >= 0 {
            let current_ci = self.base_ci[self.ci as usize].clone();
            self.base = current_ci.borrow().base;
            let top = &current_ci.borrow_mut().get_top();
            self.set_top(top);
        } else {
            self.base = 0;
            self.set_top(&(res + wanted));
        }
    }
}

impl luaState for LuaState {
    fn abs_index(&self, index: isize) -> isize {
        if index >= 0 {
            index
        } else {
            let top = self.stack.get_top();
            let base = self.base;
            top - base + index + 1
        }
    }

    fn get_top(&self) -> isize {
        self.stack.get_top() - self.base
    }

    fn get(&self, index: isize) -> LuaValue {
        self.get_value(index)
    }

    fn push(&mut self, value: LuaValue) {
        self.stack.push(value)
    }

    fn set_global(&mut self, key: &str) {
        if let LuaValue::Table(reg) = &self.registry {
            if let LuaValue::Table(g) = reg.borrow_mut().get_array(LUA_RIDX_GLOBALS) {
                let value = self.stack.pop();
                let k = LuaValue::String(key.to_string());
                g.borrow_mut().set_hash(k, value);
            }
        }
    }

    fn push_native_function(&mut self, func: fn(lua_State) -> usize) {
        let closure = LuaValue::new_native_closure(func);
        self.push(closure);
    }

    fn load(&mut self, proto: Prototype) {
        let closure = self.load_proto(Rc::new(proto));
        self.stack.push(closure);
    }

    fn call(&mut self, nargs: isize, nresults: isize) {
        self.internal_call(nargs, &mut Option::None)
    }

    fn lua_type(&self, index: isize) -> isize {
        match &self.stack.get(index) {
            LuaValue::Nil => LUA_TNIL,
            LuaValue::Boolean(_) => LUA_TBOOLEAN,
            LuaValue::Integer(_) => LUA_TNUMBER,
            LuaValue::Number(_) => LUA_TNUMBER,
            LuaValue::String(_) => LUA_TSTRING,
            LuaValue::Table(_) => LUA_TTABLE,
            LuaValue::Closure(_) => LUA_TFUNCTION,
        }
    }

    fn is_number(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TNUMBER
    }

    fn is_string(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TSTRING
    }

    fn is_function(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TFUNCTION
    }

    fn is_integer(&self, index: isize) -> bool {
        let v = &self.stack.get(index);
        if let LuaValue::Integer(_) = *v {
            true
        } else {
            false
        }
    }

    fn is_table(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TTABLE
    }

    fn is_nil(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TNIL
    }

    fn is_boolean(&self, index: isize) -> bool {
        self.lua_type(index) == LUA_TBOOLEAN
    }

    fn is_cfunction(&self, index: isize) -> bool {
        let v = &self.stack.get(index);
        if let LuaValue::Closure(f) = (*v).clone() {
            if f.borrow().function.is_some() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl LuaState {
    pub(crate) fn internal_call(
        &mut self,
        nargs: isize,
        hook: &mut Option<&mut dyn FnMut(&LuaState)>,
    ) {
        let index = self.get_top() - nargs;
        if let LuaValue::Closure(func) = self.get_value(index) {
            let ci = CallInfo::new(func.clone(), index + 1);
            self.base = ci.get_base();
            self.stack.set_top(&ci.get_top());
            self.base_ci.push(Rc::new(RefCell::new(ci)));
            self.ci += 1;

            loop {
                match self.fetch() {
                    Some(inst) => {
                        inst.execute(self);
                        match hook {
                            Some(f) => f(self),
                            None => (),
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
}
