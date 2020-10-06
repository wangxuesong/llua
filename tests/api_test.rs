use llua::api::*;
use llua::debug;

#[test]
fn push_is_test() {
    debug!("test push & is api");
    let l = luaL_newstate();
    assert_eq!(lua_gettop(l.clone()), -1);
    lua_pushnil(l.clone());
    assert_eq!(lua_gettop(l.clone()), 0);
    assert!(lua_isnil(l.clone(), -1));
    lua_pushinteger(l.clone(), 881103);
    assert_eq!(lua_gettop(l.clone()), 1);
    assert!(lua_isinteger(l.clone(), -1));
    lua_pushboolean(l.clone(), true);
    assert_eq!(lua_gettop(l.clone()), 2);
    assert!(lua_isboolean(l.clone(), -1));
    lua_pushstring(l.clone(), "sweethui");
    assert_eq!(lua_gettop(l.clone()), 3);
    assert!(lua_isstring(l.clone(), -1));
    lua_pushnumber(l.clone(), 881103.0);
    assert_eq!(lua_gettop(l.clone()), 4);
    assert!(lua_isnumber(l.clone(), -1));
}
