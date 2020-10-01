use crate::chunk::binary::{Constant, ConstantValue, Prototype};
use crate::state::{LuaClosure, LuaStack, LuaTable, LuaValue};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CallInfo {
    func: Rc<LuaClosure>,
    pc: usize,
    pub base: isize,
    pub top: isize,
}

impl CallInfo {
    pub fn new(proto: Rc<LuaClosure>, base: isize) -> Self {
        let top = base + proto.proto.max_stack_size as isize;
        CallInfo {
            func: proto,
            pc: 0,
            base,
            top,
        }
    }

    pub fn fetch(&mut self) -> Option<u32> {
        if self.pc < self.func.proto.code.len() {
            let inst = self.func.proto.code[self.pc];
            self.pc += 1;
            Some(inst)
        } else {
            None
        }
    }

    pub fn get_const(&self, index: isize) -> &Constant {
        &self.func.proto.constants[index as usize]
    }

    pub fn load_proto(&self, index: isize) -> &Rc<Prototype> {
        &self.func.proto.prototypes[index as usize]
    }

    pub fn get_base(&self) -> isize {
        self.base.clone()
    }

    pub fn get_top(&self) -> isize {
        self.top.clone()
    }
}

pub struct LuaState {
    pub stack: LuaStack,
    top: isize,
    base: isize,
    base_ci: Vec<Rc<RefCell<CallInfo>>>,
    ci: isize,
}

impl LuaState {
    pub fn new(proto: Prototype) -> LuaState {
        let closure = LuaClosure::new(Rc::new(proto));
        let ci = CallInfo::new(Rc::new(closure), 0);
        LuaState {
            stack: LuaStack::new(30),
            top: 0,
            base: 0,
            base_ci: vec![Rc::new(RefCell::new(ci))],
            ci: 0,
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
        self.stack.get(index)
    }

    pub fn load_proto(&self, index: isize) -> LuaValue {
        let proto = self.base_ci[self.ci as usize]
            .borrow()
            .load_proto(index)
            .clone();
        let mut closure = LuaClosure::new(proto.clone());
        let n = proto.upvalues.len();
        if n > 0 {
            for i in 0..n {
                if proto.upvalues[i].instack == 1 {
                    let v = self.get_value(self.base + proto.upvalues[i].idx.clone() as isize);
                    closure.upvalues.push(v);
                }
            }
        }

        LuaValue::Closure(Rc::new(closure))
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
        let x = ci.borrow().func.upvalues[index as usize].clone();
        x
    }

    pub fn precall(&mut self, a: isize, _b: isize, _c: isize) {
        if let LuaValue::Closure(func) = self.get_value(a) {
            let ci = CallInfo::new(func.clone(), a + 1);
            self.base = ci.get_base();
            self.top = ci.get_top();
            let top = self.top;
            self.set_top(&top);
            self.base_ci.push(Rc::new(RefCell::new(ci)));
            self.ci += 1;
        }
    }

    pub fn postcall(&mut self, a: isize, b: isize, _c: isize) {
        let _ci = self.base_ci.pop().unwrap();
        self.ci -= 1;

        if self.ci >= 0 {
            self.base = self.base_ci[self.ci as usize].borrow().base;
        }

        if b > 1 {
            let base = _ci.borrow().base;
            let mut index = base - 1;
            for i in a..b - 1 + a {
                self.set_value(index, self.get_value(i + base));
                index += 1;
            }
        }
    }
}
