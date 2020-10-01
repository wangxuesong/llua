use crate::chunk::binary::Prototype;
use crate::state::LuaValue;
use nom::lib::std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LuaClosure {
    pub proto: Rc<Prototype>,
    pub upvalues: Vec<LuaValue>,
}

impl PartialEq for LuaClosure {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

impl Hash for LuaClosure {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        unimplemented!()
    }
}

impl LuaClosure {
    pub fn new(proto: Rc<Prototype>) -> LuaClosure {
        LuaClosure {
            proto,
            upvalues: Vec::new(),
        }
    }
}
