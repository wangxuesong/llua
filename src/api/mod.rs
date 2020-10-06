pub mod constants;
pub mod lua_state;

pub use self::lua_state::*;
use crate::state::LuaState;
pub use crate::state::LuaValue;
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

pub fn lua_isnil(l: lua_State, idx: isize) -> bool {
    l.borrow().is_nil(lua_absindex(l.clone(), idx))
}

pub fn lua_isboolean(l: lua_State, idx: isize) -> bool {
    l.borrow().is_boolean(lua_absindex(l.clone(), idx))
}

pub fn lua_isnumber(l: lua_State, idx: isize) -> bool {
    l.borrow().is_number(lua_absindex(l.clone(), idx))
}

pub fn lua_isstring(l: lua_State, idx: isize) -> bool {
    l.borrow().is_string(lua_absindex(l.clone(), idx))
}

pub fn lua_iscfunction(l: lua_State, idx: isize) -> bool {
    l.borrow().is_cfunction(lua_absindex(l.clone(), idx))
}

pub fn lua_isinteger(l: lua_State, idx: isize) -> bool {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().is_integer(index)
}

pub fn lua_isfunction(l: lua_State, idx: isize) -> bool {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().is_function(index)
}

pub fn lua_type(l: lua_State, idx: isize) -> isize {
    let index = lua_absindex(l.clone(), idx);
    l.borrow().lua_type(index)
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

pub fn lua_pushnil(l: lua_State) {
    l.borrow_mut().push(LuaValue::Nil)
}

pub fn lua_pushboolean(l: lua_State, value: bool) {
    l.borrow_mut().push(LuaValue::Boolean(value))
}

pub fn lua_pushnumber(l: lua_State, value: f64) {
    l.borrow_mut().push(LuaValue::Number(value))
}

pub fn lua_pushinteger(l: lua_State, value: isize) {
    l.borrow_mut().push(LuaValue::Integer(value as i64))
}

pub fn lua_pushstring(l: lua_State, value: &str) {
    l.borrow_mut().push(LuaValue::String(value.to_string()))
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
