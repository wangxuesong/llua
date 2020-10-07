use llua::api::*;
use llua::debug;

#[test]
fn sample_lua() {
    let l = luaL_newstate();
    assert_eq!(lua_gettop(l.clone()), 0);
    luaL_loadfile(l.clone(), "tests/sample.out");
    assert_eq!(lua_gettop(l.clone()), 1);
    assert!(lua_isfunction(l.clone(), lua_gettop(l.clone())));
        lua_call(l.clone(), 0, 0);
    assert!(lua_isnumber(l.clone(), lua_gettop(l.clone())));
}

#[test]
fn function_test() {
    debug!("test script func.lua");
    let l = luaL_newstate();
    assert_eq!(lua_gettop(l.clone()), 0);
    assert!(lua_isnil(l.clone(), -1));
    luaL_loadfile(l.clone(), "tests/func.out");
    assert_eq!(lua_gettop(l.clone()), 1);
    assert!(lua_isfunction(l.clone(), -1));
    lua_call(l.clone(), 0,0);
    assert_eq!(l.borrow().get_top(), 3);
    assert!(lua_isnumber(l.clone(), -1));
    assert!(lua_isnumber(l.clone(), -2));
    assert!(lua_isnumber(l.clone(), -3));
    assert_eq!(lua_tointeger(l.clone(), -1), LuaValue::Integer(14));
    assert_eq!(lua_tointeger(l.clone(), -2), LuaValue::Integer(3));
    assert_eq!(lua_tointeger(l.clone(), -3), LuaValue::Integer(11));
}

#[test]
fn global_test() {
    debug!("test script global.lua");
    let l = luaL_newstate();
    lua_pushinteger(l.clone(), 1103);
    assert_eq!(lua_gettop(l.clone()), 1);
    lua_setglobal(l.clone(), "hui");
    assert_eq!(lua_gettop(l.clone()), 0);
    luaL_loadfile(l.clone(), "tests/global.out");
    assert_eq!(lua_gettop(l.clone()), 1);
    assert!(lua_isfunction(l.clone(), -1));
    lua_call(l.clone(), 0,0);
    assert_eq!(lua_gettop(l.clone()), 2);
    assert!(lua_isnumber(l.clone(), -1));
    assert!(lua_isnumber(l.clone(), -2));
    assert_eq!(lua_tointeger(l.clone(), -1), LuaValue::Integer(1103));
    assert_eq!(lua_tointeger(l.clone(), -2), LuaValue::Integer(88));
}

#[test]
fn print_test() {
    debug!("test script print.lua");
    fn print(l: lua_State) -> usize {
        let top = l.borrow().get_top();
        let arg = lua_tostring(l, top);
        assert_eq!(arg, "Hello SweetHui".to_string());
        return 0;
    }
    let l = luaL_newstate();
    assert_eq!(lua_gettop(l.clone()), 0);
    lua_pushcfunction(l.clone(), print);
    assert_eq!(lua_gettop(l.clone()), 1);
    lua_setglobal(l.clone(), "print");
    assert_eq!(lua_gettop(l.clone()), 0);
    luaL_loadfile(l.clone(), "tests/print.out");
    assert_eq!(lua_gettop(l.clone()), 1);
    assert!(lua_isfunction(l.clone(), -1));
    lua_call(l.clone(),0,0);
    assert!(lua_isnumber(l.clone(), -1));
    assert_eq!(lua_tointeger(l.clone(), -1), LuaValue::Integer(881103));
}
