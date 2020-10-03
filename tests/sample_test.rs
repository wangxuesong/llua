use llua::api::{luaL_loadfile, luaL_newstate};

#[test]
fn sample_lua() {
    assert_eq!(1, 1);
    let l = luaL_newstate();
    luaL_loadfile(l.clone(), "sample.out");
    let mut ll = l.borrow_mut();
    assert!(ll.is_function(ll.get_top()));
    ll.call(0, 0);
    assert!(ll.is_integer(ll.get_top() + 1));
}
