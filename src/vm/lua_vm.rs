use crate::state::LuaState;
use crate::vm::Instruction;

pub fn lua_vm_execute(l: &mut LuaState, func: &mut Option<&mut dyn FnMut(&LuaState)>) {
    loop {
        match l.fetch() {
            Some(inst) => {
                inst.execute(l);
                match func {
                    Some(f) => f(l),
                    None => (),
                }
            }
            None => {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::binary::{Chunk, Prototype};
    use crate::state::{LuaState, LuaTable, LuaValue};
    use crate::vm::lua_vm::lua_vm_execute;
    use std::cell::RefCell;
    use std::fs::read;
    use std::rc::Rc;

    fn read_chunk(name: &str) -> Prototype {
        let content = read(name).unwrap();
        let parse_result = Chunk::parse(content.as_slice());
        assert!(parse_result.is_ok());
        let chunk: Chunk = parse_result.unwrap().1;
        chunk.main
    }

    #[test]
    fn execute_test() {
        let proto = read_chunk("sample.out");
        let index = proto.max_stack_size.clone() as isize;
        let mut l = LuaState::new(proto);
        l.set_top(&index);
        let mut expect_index = 0;
        let expect = vec![
            //	LOADK    	0 -1	; 0
            //	ADD      	0 0 -2	; 1
            //	ADD      	0 0 -3	; 2
            //	RETURN   	0 1
            LuaValue::Integer(0),
            LuaValue::Integer(1),
            LuaValue::Integer(3),
            LuaValue::Integer(3),
        ];
        let mut cls = |l: &LuaState| {
            dbg!("hello");
            assert_eq!(l.stack.stack[0], expect[expect_index]);
            expect_index += 1;
        };
        lua_vm_execute(&mut l, &mut Some(&mut cls));
        assert_eq!(expect_index, expect.len());
    }

    #[test]
    fn local_var_test() {
        let proto = read_chunk("local_var.out");
        let index = proto.max_stack_size.clone() as isize;
        let mut l = LuaState::new(proto);
        l.set_top(&index);
        let mut expect_index = 0;
        let expect = vec![
            // LOADK    	0 -1	; 1
            // LOADK    	1 -2	; 2
            // LOADK    	2 -3	; 3
            // ADD      	3 0 1
            // ADD      	3 3 2
            // RETURN   	0 1
            (0, LuaValue::Integer(1)),
            (1, LuaValue::Integer(2)),
            (2, LuaValue::Integer(3)),
            (3, LuaValue::Integer(3)),
            (3, LuaValue::Integer(6)),
            (3, LuaValue::Integer(6)),
        ];
        let mut expect_fun = |l: &LuaState| {
            dbg!("assert local variable");
            let (i, v) = expect[expect_index].clone();
            assert_eq!(l.stack.stack[i], v);
            expect_index += 1;
        };
        lua_vm_execute(&mut l, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, expect.len());
    }

    #[test]
    fn table_test() {
        let proto = read_chunk("table.out");
        let index = proto.max_stack_size.clone() as isize;
        let mut l = LuaState::new(proto);
        l.set_top(&index);
        let mut expect_index = 0;
        let mut expect_closure: Vec<Box<dyn FnMut(&LuaState)>> = Vec::new();
        // 1	[1]	NEWTABLE 	0 3 0
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(
                l.stack.stack[0],
                LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(3, 0))))
            )
        }));
        // 2	[1]	LOADK    	1 -1	; 88
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[1], LuaValue::Integer(88));
        }));
        // 3	[1]	LOADK    	2 -2	; 11
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[2], LuaValue::Integer(11))
        }));
        // 4	[1]	LOADK    	3 -3	; 3
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[3], LuaValue::Integer(03))
        }));
        // 5	[1]	SETLIST  	0 3 1	; 1
        expect_closure.push(Box::new(|l: &LuaState| {
            if let LuaValue::Table(table) = &l.stack.stack[0] {
                assert_eq!(table.borrow_mut().len(), 3);
                assert_eq!(table.borrow_mut().get_array(1), LuaValue::Integer(88));
                assert_eq!(table.borrow_mut().get_array(2), LuaValue::Integer(11));
                assert_eq!(table.borrow_mut().get_array(3), LuaValue::Integer(03));
            } else {
                assert!(false)
            }
        }));
        // 6	[2]	SETTABLE 	0 -4 -5	; "sweethui" 881103
        expect_closure.push(Box::new(|l: &LuaState| {
            if let LuaValue::Table(table) = &l.stack.stack[0] {
                // assert_eq!(table.borrow_mut().len(), 3);
                assert_eq!(
                    table
                        .borrow_mut()
                        .get_hash(LuaValue::String("sweethui".to_string())),
                    LuaValue::Integer(881103)
                );
            } else {
                assert!(false)
            }
        }));
        // 7	[3]	GETTABLE 	1 0 -4	; "sweethui"
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[1], LuaValue::Integer(881103));
        }));
        // 8	[4]	GETTABLE 	2 0 -6	; 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[2], LuaValue::Integer(88));
        }));
        // 9	[2]	RETURN   	0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[2], LuaValue::Integer(88));
        }));
        let mut expect_fun = |l: &LuaState| {
            dbg!("assert table");
            let func = &mut expect_closure[expect_index];
            func(l);
            expect_index += 1;
        };
        lua_vm_execute(&mut l, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, expect_closure.len());
    }

    #[test]
    fn function_test() {
        let proto = read_chunk("func.out");
        let index = proto.max_stack_size.clone() as isize;
        let mut l = LuaState::new(proto);
        l.set_top(&index);
        let mut expect_index = 0;
        let mut expect_closure: Vec<Box<dyn FnMut(&LuaState)>> = Vec::new();
        // 1	[9] 	CLOSURE  	0 0	; 0x7fd20d4063c0
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("CLOSURE  	0 0");
            if let LuaValue::Function(_) = l.stack.stack[0] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 2	[11]	MOVE     	1 0
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("MOVE     	1 0");
            if let LuaValue::Function(_) = l.stack.stack[1] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 3	[11]	LOADK    	2 -1	; 11
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("LOADK    	2 -1	; 11");
            assert_eq!(l.stack.stack[2], LuaValue::Integer(11))
        }));
        // 4	[11]	LOADK    	3 -2	; 3
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("LOADK    	3 -2	; 3");
            assert_eq!(l.stack.stack[3], LuaValue::Integer(3))
        }));
        // 5	[11]	CALL     	1 3 2
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("CALL     	1 3 2");
            if let LuaValue::Function(_) = l.stack.stack[1] {
            } else {
                assert!(false, "expect function")
            }
            assert_eq!(l.stack.stack[2], LuaValue::Integer(11));
            assert_eq!(l.stack.stack[3], LuaValue::Integer(3));
        }));
        // 1	[8]	ADD      	2 0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("ADD      	2 0 1");
            assert_eq!(l.stack.stack[4], LuaValue::Integer(14))
        }));
        // 2	[8]	RETURN   	2 2
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("RETURN   	2 2");
            assert_eq!(l.stack.stack[4], LuaValue::Integer(14))
        }));
        // 6	[11]	RETURN   	0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            dbg!("RETURN   	0 1");
            assert_eq!(l.stack.stack[4], LuaValue::Integer(14))
        }));

        let mut expect_fun = |l: &LuaState| {
            let func = &mut expect_closure[expect_index];
            func(l);
            expect_index += 1;
        };
        lua_vm_execute(&mut l, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, 8);
    }
}
