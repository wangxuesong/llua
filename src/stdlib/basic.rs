use crate::api::*;

pub fn basic_print(l: lua_State) -> usize {
    let argc = l.borrow().get_top();
    for i in 1..=argc {
        if lua_isstring(l.clone(), i) {
            print!("{}", lua_tostring(l.clone(), i));
        }
    }
    println!();
    0
}
