use crate::state::{LuaState, LuaValue};
use crate::vm::Instruction;

pub fn get_upvalue(i: u32, l: &mut LuaState) {
    dbg!(i.opname());
    let (a, b, _) = i.abc();
    l.set_value(a, l.get_upvalue(b));
}

pub fn get_table_upvalue(i: u32, l: &mut LuaState) {
    dbg!(i.opname());
    let (a, b, c) = i.abc();
    let key = l.get_rk(c);
    if let LuaValue::Table(t) = l.get_upvalue(b) {
        let value = t.borrow_mut().get(key);
        l.set_value(a, value);
    }
}
