pub const LUA_TNIL: isize = 0;
pub const LUA_TBOOLEAN: isize = 1;
pub const LUA_TLIGHTUSERDATA: isize = 2;
pub const LUA_TNUMBER: isize = 3;
pub const LUA_TSTRING: isize = 4;
pub const LUA_TTABLE: isize = 5;
pub const LUA_TFUNCTION: isize = 6;
pub const LUA_TUSERDATA: isize = 7;
pub const LUA_TTHREAD: isize = 8;

pub const LUA_MINSTACK: usize = 20;
pub const LUAI_MAXSTACK: usize = 1000000;
pub const LUA_REGISTRYINDEX: isize = -(LUAI_MAXSTACK as isize) - 1000;
pub const LUA_RIDX_GLOBALS: isize = 2;
