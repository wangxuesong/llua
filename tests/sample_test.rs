use llua::api::*;

#[test]
fn sample_lua() {
    let l = luaL_newstate();
    luaL_loadfile(l.clone(), "tests/sample.out");
    assert!(lua_isfunction(l.clone(), lua_gettop(l.clone())));
    {
        let mut ll = l.borrow_mut();
        ll.call(0, 0);
    }
    assert!(lua_isnumber(l.clone(), lua_gettop(l.clone())));
}

#[test]
fn function_test() {
    dbg!("test script func.lua");
    let l = luaL_newstate();
    luaL_loadfile(l.clone(), "tests/func.out");
    assert!(lua_isfunction(l.clone(), -1));
    {
        let mut ll = l.borrow_mut();
        ll.call(0, 0);
    }
    assert!(lua_isnumber(l.clone(), -1));
    assert!(lua_isnumber(l.clone(), -2));
    assert!(lua_isnumber(l.clone(), -3));
    assert_eq!(lua_tointeger(l.clone(), -1), LuaValue::Integer(14));
    assert_eq!(lua_tointeger(l.clone(), -2), LuaValue::Integer(3));
    assert_eq!(lua_tointeger(l.clone(), -3), LuaValue::Integer(11));
}

#[test]
fn global_test() {
    dbg!("test script global.lua");
    let l = luaL_newstate();
    lua_pushinteger(l.clone(), 1103);
    lua_setglobal(l.clone(), "hui");
    luaL_loadfile(l.clone(), "tests/global.out");
    assert!(lua_isfunction(l.clone(), -1));
    {
        let mut ll = l.borrow_mut();
        ll.call(0, 0);
    }
    assert!(lua_isnumber(l.clone(), -1));
    assert!(lua_isnumber(l.clone(), -2));
    assert_eq!(lua_tointeger(l.clone(), -1), LuaValue::Integer(1103));
    assert_eq!(lua_tointeger(l.clone(), -2), LuaValue::Integer(88));
}
