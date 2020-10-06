use crate::state::lua_value::LuaValue;

#[derive(Clone)]
pub struct LuaStack {
    pub stack: Vec<LuaValue>,
    top: isize,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        let stack = Vec::with_capacity(size);
        // stack.push(LuaValue::Nil);
        LuaStack { stack, top: 0 }
    }

    pub fn get_top(&self) -> isize {
        self.stack.len() as isize - 1
    }

    pub fn set_top(&mut self, index: &isize) {
        if *index as usize >= self.stack.len() {
            let size = *index as usize - self.stack.len();
            for _ in 0..size {
                self.stack.push(LuaValue::Nil);
            }
        } else {
            let mut i = self.stack.len();
            while i > *index as usize {
                self.stack.pop();
                i -= 1;
            }
        };
        self.top = self.stack.len() as isize;
    }

    pub fn set_size(&mut self, index: isize) {
        if index > self.stack.len() as isize {
            let top = self.stack.len() as isize;
            for _ in top..index {
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
