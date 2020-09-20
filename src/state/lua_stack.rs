use crate::state::lua_value::LuaValue;

pub struct LuaStack {
    pub stack: Vec<LuaValue>,
    top: isize,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        LuaStack {
            stack: Vec::with_capacity(size),
            top: 0,
        }
    }

    pub fn set_top(&mut self, index: &isize) {
        for i in 0..*index {
            self.stack.push(LuaValue::Nil);
        }
    }

    pub fn push(&mut self, value: LuaValue) {
        self.stack.push(value);
        self.top += 1;
    }

    pub fn pop(&mut self) -> LuaValue {
        self.top -= 1;
        self.stack.pop().unwrap()
    }

    pub fn get(&mut self, index: isize) -> LuaValue {
        self.stack[index as usize].clone()
    }

    pub fn set(&mut self, index: isize, value: LuaValue) {
        self.stack[index as usize] = value;
    }
}
