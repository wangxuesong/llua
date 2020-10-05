pub mod lua_state;

pub use self::lua_state::*;
pub use crate::state::LuaValue;
use std::cell::RefCell;
use std::rc::Rc;

// state manipulation

#[allow(non_snake_case)]
pub fn luaL_newstate() -> lua_State {
    let l = crate::state::LuaState::new();
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

// Comparison and arithmetic functions

// push functions (C -> stack)

// get functions (Lua -> stack)

// set functions (stack -> Lua)

// 'load' and 'call' functions (load and run Lua code)

pub fn lua_call(l: lua_State, nargs: isize, nresults: isize) {
    l.borrow_mut().call(nargs, nresults)
}

// coroutine functions

// garbage-collection function and options

// miscellaneous functions
