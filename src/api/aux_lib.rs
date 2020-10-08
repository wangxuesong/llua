// Auxiliary functions

use crate::api::*;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(non_camel_case_types)]
pub struct luaL_Reg {
    pub(crate) name: &'static str,
    pub(crate) func: lua_CFunction,
}

#[allow(non_snake_case)]
pub fn luaL_newstate() -> lua_State {
    let l = crate::state::LuaState::new();
    Rc::new(RefCell::new(l))
}

#[allow(non_snake_case)]
pub fn luaL_loadfile(l: lua_State, filename: &str) {
    let proto = crate::vm::read_chunk(filename);
    l.borrow_mut().load(proto);
}

#[allow(non_snake_case)]
pub fn luaL_setfuncs(l: lua_State, regs: &[luaL_Reg]) {
    for r in regs {
        lua_pushcfunction(l.clone(), r.func);
        lua_setglobal(l.clone(), r.name);
    }
}
