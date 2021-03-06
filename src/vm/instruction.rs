use super::opcodes::*;
use crate::state::{LuaState, LuaValue};
use crate::vm::upvalue;

const MAXARG_BX: isize = (1 << 18) - 1; // 262143
const MAXARG_SBX: isize = MAXARG_BX >> 1; // 131071

/*
 31       22       13       5    0
  +-------+^------+-^-----+-^-----
  |b=9bits |c=9bits |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |    bx=18bits    |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |   sbx=18bits    |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |    ax=26bits            |op=6|
  +-------+^------+-^-----+-^-----
 31      23      15       7      0
*/
pub trait Instruction {
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn opcode(self) -> u8;
    fn abc(self) -> (isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
    fn execute(self, l: &mut LuaState);
}

impl Instruction for u32 {
    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name
    }

    fn opmode(self) -> u8 {
        OPCODES[self.opcode() as usize].opmode
    }

    fn b_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].bmode
    }

    fn c_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].cmode
    }

    fn opcode(self) -> u8 {
        self as u8 & 0x3F
    }

    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_SBX)
    }

    fn ax(self) -> isize {
        (self >> 6) as isize
    }

    fn execute(self, l: &mut LuaState) {
        match self.opcode() {
            OP_MOVE => {
                debug!(self.opname());
                let (a, b, _) = self.abc();
                let v = l.get_register(b);
                l.set_register(a, v);
            }
            OP_LOADK => {
                debug!(self.opname());
                let (a, bx) = self.a_bx();

                let v = l.get_const(bx);
                l.set_register(a, v);
            }
            OP_GETUPVAL => upvalue::get_upvalue(self, l),
            OP_GETTABUP => upvalue::get_table_upvalue(self, l),
            OP_GETTABLE => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                let value = l.get_register(b);
                assert!(value.is_table());
                if let LuaValue::Table(table) = value {
                    let v = table.borrow().get(l.get_rk(c));
                    l.set_register(a, v)
                }
            }
            OP_SETTABLE => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                let value = l.get_register(a);
                assert!(value.is_table());
                if let LuaValue::Table(table) = value {
                    for _i in 1..=b {
                        table.borrow_mut().set_hash(l.get_rk(b), l.get_rk(c));
                    }
                }
            }
            OP_NEWTABLE => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                let v = l.create_table(b, c);
                l.set_register(a, v);
            }
            OP_ADD => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                if let LuaValue::Integer(b_value) = l.get_rk(b) {
                    if let LuaValue::Integer(c_value) = l.get_rk(c) {
                        let add = b_value + c_value;
                        l.set_register(a, LuaValue::Integer(add));
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            OP_CALL => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                if let LuaValue::Closure(_) = l.get_register(a) {
                    l.precall(a, b, c);
                }
            }
            OP_RETURN => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                l.postcall(a, b, c);
            }
            OP_SETLIST => {
                debug!(self.opname());
                let (a, b, c) = self.abc();
                let last = (c - 1) * 50/* LFIELDS_PER_FLUSH */ + b;
                let value = l.get_register(a);
                assert!(value.is_table());
                if let LuaValue::Table(table) = value {
                    for i in 1..=b {
                        table
                            .borrow_mut()
                            .set_array(i, l.get_register(last - b + i))
                    }
                }
            }
            OP_CLOSURE => {
                debug!(self.opname());
                let (a, b) = self.a_bx();
                let proto = l.get_subproto(b);
                let closure = l.load_proto(proto);
                l.set_register(a, closure);
            }
            _ => {
                debug!(self.opname());
                unimplemented!()
            }
        }
    }
}
