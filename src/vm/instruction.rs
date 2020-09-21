use super::opcodes::*;
use crate::state::{LuaState, LuaValue};
use nom::dbg_dmp;

const MAXARG_BX: isize = (1 << 18) - 1;
// 262143
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
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                let v = l.get_value(b);
                l.set_value(a, v);
            }
            OP_LOADK => {
                dbg!(self.opname());
                let (mut a, bx) = self.a_bx();

                let v = l.get_const(bx);
                l.set_value(a, v);
            }
            OP_GETTABLE => {
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                let value = l.get_value(b);
                assert!(value.is_table());
                if let LuaValue::Table(mut table) = value {
                    let v = table.borrow().get(l.get_rk(c));
                    l.set_value(a, v)
                }
            }
            OP_SETTABLE => {
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                let value = l.get_value(a);
                assert!(value.is_table());
                if let LuaValue::Table(mut table) = value {
                    for i in 1..=b {
                        table.borrow_mut().set_hash(l.get_rk(b), l.get_rk(c));
                    }
                }
            }
            OP_NEWTABLE => {
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                let v = l.create_table(b, c);
                l.set_value(a, v);
            }
            OP_ADD => {
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                if let LuaValue::Integer(b_value) = l.get_rk(b) {
                    if let LuaValue::Integer(c_value) = l.get_rk(c) {
                        let add = b_value + c_value;
                        l.set_value(a, LuaValue::Integer(add));
                    }
                }
            }
            OP_SETLIST => {
                dbg!(self.opname());
                let (a, b, c) = self.abc();
                let last = (c - 1) * 50/* LFIELDS_PER_FLUSH */ + b;
                let value = l.get_value(a);
                assert!(value.is_table());
                if let LuaValue::Table(mut table) = value {
                    for i in 1..=b {
                        table.borrow_mut().set_array(i, l.get_value(last - b + i))
                    }
                }
            }
            OP_CLOSURE => {
                dbg!(self.opname());
                let (a, b) = self.a_bx();
                let proto = l.load_proto(b);
                l.set_value(a, proto);
            }
            _ => {
                dbg!(self.opname());
                unimplemented!()
            }
        }
    }
}
