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

    pub fn set_top(&mut self, _index: &isize) {
        if *_index as usize >= self.stack.len() {
            let size = *_index as usize - self.stack.len();
            for _ in 0..size {
                self.stack.push(LuaValue::Nil);
            }
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

    pub fn get(&self, index: isize) -> LuaValue {
        self.stack[index as usize].clone()
    }

    pub fn set(&mut self, index: isize, value: LuaValue) {
        self.stack[index as usize] = value;
    }
}
