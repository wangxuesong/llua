pub mod lua_state;

pub use self::lua_state::*;
use crate::state::LuaState;
pub use crate::state::{LuaValue, LUA_RIDX_GLOBALS};
use std::cell::RefCell;
use std::rc::Rc;

#[allow(non_camel_case_types)]
pub type lua_CFunction = fn(lua_State) -> usize;

// state manipulation

#[allow(non_snake_case)]
pub fn luaL_newstate() -> lua_State {
    let l = crate::state::LuaState::new();
    Rc::new(RefCell::new(l))
}

pub fn create_state(l: LuaState) -> lua_State {
    Rc::new(RefCell::new(l))
}

// basic stack manipulation

pub fn lua_absindex(l: lua_State, idx: isize) -> isize {
    l.borrow().abs_index(idx)
}

pub fn lua_gettop(l: lua_State) -> isize {
    l.borrow().get_top()
}

// access functions (stack -> C)

pub fn lua_isinteger(l: lua_State, idx: isize) -> bool {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().is_integer(index)
}

pub fn lua_isnumber(l: lua_State, idx: isize) -> bool {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().is_integer(index)
}

pub fn lua_isfunction(l: lua_State, idx: isize) -> bool {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().is_function(index)
}

pub fn lua_tointeger(l: lua_State, idx: isize) -> LuaValue {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().get(index)
}

pub fn lua_tostring(l: lua_State, idx: isize) -> String {
    let index = lua_absindex(l.clone(), idx);
    if let LuaValue::String(s) = l.borrow().get(index) {
        s
    } else {
        "".to_string()
    }
}

// Comparison and arithmetic functions

// push functions (C -> stack)

pub fn lua_pushinteger(l: lua_State, value: isize) {
    l.borrow_mut().push(LuaValue::Integer(value as i64))
}

pub fn lua_pushcfunction(l: lua_State, func: lua_CFunction) {
    l.borrow_mut().push_native_function(func)
}

// get functions (Lua -> stack)

// set functions (stack -> Lua)

pub fn lua_setglobal(l: lua_State, value: &str) {
    l.borrow_mut().set_global(value)
}

// 'load' and 'call' functions (load and run Lua code)

pub fn lua_call(l: lua_State, nargs: isize, nresults: isize) {
    l.borrow_mut().call(nargs, nresults)
}

// coroutine functions

// garbage-collection function and options

// miscellaneous functions
