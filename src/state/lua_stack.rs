use crate::state::lua_value::LuaValue;

#[derive(Clone)]
pub struct LuaStack {
    pub stack: Vec<LuaValue>,
    top: isize,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        let mut stack = Vec::with_capacity(size);
        stack.push(LuaValue::Nil);
        LuaStack { stack, top: 1 }
    }

    pub fn get_top(&self) -> isize {
        self.top
    }

    pub fn set_top(&mut self, index: &isize) {
        let idx = *index;
        if idx >= self.top {
            let size = idx - self.top;
            for _ in 0..size {
                self.stack.push(LuaValue::Nil);
                self.top += 1;
            }
        } else {
            let mut i = self.top;
            while i > idx {
                self.stack.pop();
                i -= 1;
                self.top -= 1;
            }
        };
    }

    pub fn set_size(&mut self, index: isize) {
        if index > self.stack.len() as isize {
            let top = (self.stack.len() - 1) as isize;
            for _ in top..index {
                self.stack.push(LuaValue::Nil);
                self.top += 1;
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
