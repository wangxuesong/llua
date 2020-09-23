use crate::chunk::binary::Prototype;
use nom::lib::std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LuaFunction {
    pub proto: Rc<Prototype>,
}

impl PartialEq for LuaFunction {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl Hash for LuaFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unimplemented!()
    }
}

impl LuaFunction {
    pub fn new(proto: Rc<Prototype>) -> LuaFunction {
        LuaFunction { proto }
    }
}
