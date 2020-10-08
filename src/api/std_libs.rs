// Lua standard libraries

use super::lua_State;
use crate::api::*;
use crate::stdlib::basic_print;

const BASE_FUNCTION: &'static [luaL_Reg] = &[register_lib_function("print", basic_print)];

const fn register_lib_function(name: &'static str, func: lua_CFunction) -> luaL_Reg {
    luaL_Reg { name, func }
}

pub fn luaopen_base(l: lua_State) -> isize {
    lua_pushglobaltable(l.clone());
    luaL_setfuncs(l.clone(), BASE_FUNCTION);
    lua_pushvalue(l.clone(), -1);
    lua_setfield(l, -2, "_G");
    0
}
