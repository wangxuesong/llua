use crate::api::lua_CFunction;
use crate::chunk::binary::Prototype;
use crate::state::LuaValue;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(non_camel_case_types)]
pub trait luaState {
    fn abs_index(&self, idx: isize) -> isize;
    fn get_top(&self) -> isize;

    fn get(&self, index: isize) -> LuaValue;
    fn push(&mut self, value: LuaValue);
    fn pop(&mut self, n: isize);
    fn pushvalue(&mut self, index: isize);

    fn get_global(&mut self, name: &str);
    fn raw_geti(&mut self, idx: isize, n: isize);

    fn set_global(&mut self, value: &str);
    fn set_field(&mut self, index: isize, name: &str);

    fn push_native_function(&mut self, func: lua_CFunction);

    fn load(&mut self, proto: Prototype);
    fn call(&mut self, nargs: isize, nresults: isize);

    fn lua_type(&self, index: isize) -> isize;
    fn is_number(&self, index: isize) -> bool;
    fn is_string(&self, index: isize) -> bool;
    fn is_cfunction(&self, index: isize) -> bool;
    fn is_integer(&self, index: isize) -> bool;
    fn is_table(&self, index: isize) -> bool;
    fn is_nil(&self, index: isize) -> bool;
    fn is_boolean(&self, index: isize) -> bool;
    fn is_function(&self, index: isize) -> bool;
}

#[allow(non_camel_case_types)]
pub type lua_State = Rc<RefCell<dyn luaState>>;
