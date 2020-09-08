use crate::chunk::binary::{Constant, ConstantValue, Prototype};
use crate::state::{LuaStack, LuaValue};
use crate::vm::Instruction;

pub struct LuaState {
    pub stack: LuaStack,
    pc: isize,
    pub proto: Prototype,
}

impl LuaState {
    pub fn new(proto: Prototype) -> LuaState {
        LuaState {
            stack: LuaStack::new(30),
            pc: 0,
            proto,
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
        let v = match c.const_value {
            ConstantValue::Nil => LuaValue::Nil,
            ConstantValue::Integer(v) => LuaValue::Integer(v),
            _ => {
                dbg!(&c.const_value);
                unimplemented!()
            }
        };
        self.stack.push(v.clone());
        v
    }

    pub fn get_rk(&mut self, index: isize) -> LuaValue {
        if index > 0xFF {
            self.get_const(index - 0xFF - 1)
        } else {
            let v = self.stack.stack[index as usize].clone();
            self.stack.push(v.clone());
            v
        }
    }

    pub fn set_value(&mut self, index: isize) {
        let value = self.stack.pop();
        self.stack.set(index, value);
    }
}
