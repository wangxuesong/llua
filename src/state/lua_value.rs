use crate::api::lua_CFunction;
use crate::chunk::binary::Prototype;
use crate::state::{LuaClosure, LuaTable};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(Rc<RefCell<LuaTable>>),
    Closure(Rc<RefCell<LuaClosure>>),
}

impl Eq for LuaValue {}

impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => 0.hash(state),
            LuaValue::Boolean(v) => v.hash(state),
            LuaValue::Integer(v) => v.hash(state),
            LuaValue::Number(v) => v.to_be_bytes().hash(state),
            LuaValue::String(v) => v.hash(state),
            LuaValue::Table(v) => v.borrow_mut().hash(state),
            LuaValue::Closure(v) => v.borrow_mut().hash(state),
        }
    }
}

impl LuaValue {
    pub fn new_lua_closure(proto: Prototype) -> LuaValue {
        LuaValue::Closure(Rc::new(RefCell::new(LuaClosure::new(Rc::new(proto)))))
    }

    pub fn new_native_closure(func: lua_CFunction) -> LuaValue {
        LuaValue::Closure(Rc::new(RefCell::new(LuaClosure::new_native(func))))
    }

    pub fn new_table(array_size: usize, hash_size: usize) -> LuaValue {
        LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(array_size, hash_size))))
    }

    pub fn is_table(&self) -> bool {
        match self {
            LuaValue::Table(_) => true,
            _ => false,
        }
    }
}
