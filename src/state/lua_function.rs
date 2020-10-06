use crate::api::lua_CFunction;
use crate::chunk::binary::Prototype;
use crate::state::LuaValue;
use nom::lib::std::fmt::{Debug, Formatter};
use nom::lib::std::hash::Hash;
use std::fmt;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone)]
pub struct LuaClosure {
    pub proto: Rc<Prototype>,
    pub function: Option<lua_CFunction>,
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

impl Debug for LuaClosure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl LuaClosure {
    pub fn new(proto: Rc<Prototype>) -> LuaClosure {
        LuaClosure {
            proto,
            function: None,
            upvalues: Vec::new(),
        }
    }

    pub fn new_native(func: lua_CFunction) -> LuaClosure {
        LuaClosure {
            proto: Rc::new(Prototype::new()),
            function: Some(func),
            upvalues: Vec::new(),
        }
    }
}
