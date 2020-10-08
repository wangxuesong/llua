use llua::api::*;
use llua::debug;

#[test]
fn base_library() {
    debug!("test base library function");
    let l = luaL_newstate();
    luaopen_base(l.clone());
    lua_getglobal(l.clone(), "print");
    assert!(lua_iscfunction(l.clone(), -1));
    lua_pop(l.clone(), 1);
    lua_getglobal(l.clone(), "_G");
    assert!(lua_istable(l.clone(), -1));
}
