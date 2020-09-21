use crate::chunk::binary::{Constant, ConstantValue, Prototype};
use crate::state::{LuaFunction, LuaStack, LuaTable, LuaValue};
use crate::vm::Instruction;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LuaState {
    pub stack: LuaStack,
    pc: isize,
    pub proto: Rc<Prototype>,
}

impl LuaState {
    pub fn new(proto: Prototype) -> LuaState {
        LuaState {
            stack: LuaStack::new(30),
            pc: 0,
            proto: Rc::new(proto),
        }
    }

    pub fn set_top(&mut self, index: &isize) {
        self.stack.set_top(index)
    }

    pub fn fetch(&mut self) -> u32 {
        let inst = self.proto.code[self.pc as usize];
        self.pc += 1;
        inst
    }

    pub fn get_const(&mut self, index: isize) -> LuaValue {
        let c = &self.proto.constants[index as usize];
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
            let v = self.stack.stack[index as usize].clone();
            v
        }
    }

    pub fn get_value(&mut self, index: isize) -> LuaValue {
        self.stack.get(index)
    }

    pub fn load_proto(&self, index: isize) -> LuaValue {
        let proto = self.proto.prototypes[index as usize].clone();
        LuaValue::Function(Rc::new(LuaFunction::new(proto)))
    }

    pub fn create_table(&mut self, array_size: isize, hash_size: isize) -> LuaValue {
        LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(
            array_size as usize,
            hash_size as usize,
        ))))
    }

    pub fn set_value(&mut self, index: isize, value: LuaValue) {
        self.stack.set(index, value);
    }
}
