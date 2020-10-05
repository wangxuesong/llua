use llua::api::{luaL_loadfile, luaL_newstate, LuaValue};

#[test]
fn sample_lua() {
    let l = luaL_newstate();
    luaL_loadfile(l.clone(), "tests/sample.out");
    let mut ll = l.borrow_mut();
    assert!(ll.is_function(ll.get_top()));
    ll.call(0, 0);
    assert!(ll.is_integer(ll.get_top()));
}

#[test]
fn function_test() {
    dbg!("test script func.lua");
    let l = luaL_newstate();
    luaL_loadfile(l.clone(), "tests/func.out");
    let mut ll = l.borrow_mut();
    assert!(ll.is_function(ll.get_top()));
    ll.call(0, 0);
    assert!(ll.is_integer(ll.get_top()));
    assert_eq!(ll.get(ll.get_top()), LuaValue::Integer(14));
}
