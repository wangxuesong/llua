use crate::chunk::binary::{Chunk, Prototype};
use crate::state::LuaState;
use crate::vm::Instruction;
use std::fs::read;

#[allow(dead_code)]
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

pub fn read_chunk(name: &str) -> Prototype {
    let content = read(name).unwrap();
    let parse_result = Chunk::parse(content.as_slice());
    assert!(parse_result.is_ok());
    let chunk: Chunk = parse_result.unwrap().1;
    chunk.main
}

#[cfg(test)]
mod tests {
    use crate::api::luaState;
    use crate::state::{LuaState, LuaTable, LuaValue};
    use crate::vm::lua_vm::read_chunk;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn execute_test() {
        let proto = read_chunk("sample.out");
        let mut l = LuaState::new();
        let closure = LuaValue::new_lua_closure(proto);
        l.push(closure);
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
            debug!("hello");
            assert_eq!(
                l.stack.stack[2], expect[expect_index],
                "expect index {}",
                expect_index
            );
            expect_index += 1;
        };
        l.internal_call(0, &mut Some(&mut cls));
        assert_eq!(expect_index, expect.len());
    }

    #[test]
    fn local_var_test() {
        let proto = read_chunk("local_var.out");
        let mut l = LuaState::new();
        let closure = LuaValue::new_lua_closure(proto);
        l.push(closure);
        let mut expect_index = 0;
        let expect = vec![
            // LOADK    	0 -1	; 1
            // LOADK    	1 -2	; 2
            // LOADK    	2 -3	; 3
            // ADD      	3 0 1
            // ADD      	3 3 2
            // RETURN   	0 1
            (1, LuaValue::Integer(1)),
            (2, LuaValue::Integer(2)),
            (3, LuaValue::Integer(3)),
            (4, LuaValue::Integer(3)),
            (4, LuaValue::Integer(6)),
            (4, LuaValue::Integer(6)),
        ];
        let mut expect_fun = |l: &LuaState| {
            debug!("assert local variable");
            let (i, v) = expect[expect_index].clone();
            assert_eq!(
                l.stack.stack[i + 1],
                v,
                "register {} with expect index {}",
                i,
                expect_index
            );
            expect_index += 1;
        };
        l.internal_call(0, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, expect.len());
    }

    #[test]
    fn table_test() {
        let proto = read_chunk("table.out");
        let mut l = LuaState::new();
        let closure = LuaValue::new_lua_closure(proto);
        l.push(closure);
        let mut expect_index = 0;
        let mut expect_closure: Vec<Box<dyn FnMut(&LuaState)>> = Vec::new();
        // 1	[1]	NEWTABLE 	0 3 0
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(
                l.stack.stack[2],
                LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(3, 0))))
            )
        }));
        // 2	[1]	LOADK    	1 -1	; 88
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[3], LuaValue::Integer(88));
        }));
        // 3	[1]	LOADK    	2 -2	; 11
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[4], LuaValue::Integer(11))
        }));
        // 4	[1]	LOADK    	3 -3	; 3
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[5], LuaValue::Integer(03))
        }));
        // 5	[1]	SETLIST  	0 3 1	; 1
        expect_closure.push(Box::new(|l: &LuaState| {
            if let LuaValue::Table(table) = &l.stack.stack[2] {
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
            if let LuaValue::Table(table) = &l.stack.stack[2] {
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
            assert_eq!(l.stack.stack[3], LuaValue::Integer(881103));
        }));
        // 8	[4]	GETTABLE 	2 0 -6	; 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[4], LuaValue::Integer(88));
        }));
        // 9	[2]	RETURN   	0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[4], LuaValue::Integer(88));
        }));
        let mut expect_fun = |l: &LuaState| {
            debug!("assert table");
            let func = &mut expect_closure[expect_index];
            func(l);
            expect_index += 1;
        };
        l.internal_call(0, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, expect_closure.len());
    }

    #[test]
    fn function_test() {
        let proto = read_chunk("func.out");
        let mut l = LuaState::new();
        let closure = LuaValue::new_lua_closure(proto);
        l.push(closure);
        let mut expect_index = 0;
        let mut expect_closure: Vec<Box<dyn FnMut(&LuaState)>> = Vec::new();
        // 1	[9] 	CLOSURE  	0 0	; 0x7fd20d4063c0
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("CLOSURE  	0 0");
            if let LuaValue::Closure(_) = l.stack.stack[2] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 2	[11]	MOVE     	1 0
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("MOVE     	1 0");
            if let LuaValue::Closure(_) = l.stack.stack[3] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 3	[11]	LOADK    	2 -1	; 11
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("LOADK    	2 -1	; 11");
            assert_eq!(l.stack.stack[4], LuaValue::Integer(11))
        }));
        // 4	[11]	LOADK    	3 -2	; 3
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("LOADK    	3 -2	; 3");
            assert_eq!(l.stack.stack[5], LuaValue::Integer(3))
        }));
        // 5	[11]	CALL     	1 3 2
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("CALL     	1 3 2");
            if let LuaValue::Closure(_) = l.stack.stack[3] {
            } else {
                assert!(false, "expect function")
            }
            assert_eq!(l.stack.stack[4], LuaValue::Integer(11));
            assert_eq!(l.stack.stack[5], LuaValue::Integer(3));
        }));
        // 1	[8]	ADD      	2 0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("ADD      	2 0 1");
            assert_eq!(l.stack.stack[6], LuaValue::Integer(14))
        }));
        // 2	[8]	RETURN   	2 2
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("RETURN   	2 2");
            assert_eq!(l.stack.stack[3], LuaValue::Integer(14))
        }));
        // 6	[11]	RETURN   	0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("RETURN   	0 1");
            assert_eq!(l.stack.stack[3], LuaValue::Integer(14))
        }));

        let mut expect_fun = |l: &LuaState| {
            let func = &mut expect_closure[expect_index];
            func(l);
            expect_index += 1;
        };
        l.internal_call(0, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, 8);
    }

    #[test]
    fn upvalue_test() {
        let proto = read_chunk("upvalue.out");
        let mut l = LuaState::new();
        let closure = LuaValue::new_lua_closure(proto);
        l.push(closure);
        let mut expect_index = 0;
        let mut expect_closure: Vec<Box<dyn FnMut(&LuaState)>> = Vec::new();
        // 1	[6] 	LOADK    	0 -1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[2], LuaValue::Integer(88));
        }));
        // 2	[6] 	LOADK    	1 -2
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[3], LuaValue::Integer(11));
        }));
        // 3	[11]	CLOSURE  	2 0
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("CLOSURE  	2 0");
            if let LuaValue::Closure(_) = l.stack.stack[4] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 4	[12]	MOVE     	3 2
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("MOVE     	3 2");
            if let LuaValue::Closure(_) = l.stack.stack[5] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 5	[12]	CALL     	3 1 1
        expect_closure.push(Box::new(|l: &LuaState| {
            debug!("CALL     	3 1 1");
            if let LuaValue::Closure(_) = l.stack.stack[5] {
            } else {
                assert!(false, "expect function")
            }
        }));
        // 1	[8] 	GETUPVAL 	0 0
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[6], LuaValue::Integer(88));
        }));
        // 2	[9] 	GETUPVAL 	1 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[7], LuaValue::Integer(11));
        }));
        // 3	[10]	MOVE     	2 0
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[8], LuaValue::Integer(88));
        }));
        // 4	[10]	MOVE     	3 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[9], LuaValue::Integer(11));
        }));
        // 5	[10]	RETURN   	2 3
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[5], LuaValue::Integer(88));
            assert_eq!(l.stack.stack[6], LuaValue::Integer(11));
        }));
        // 6	[11]	RETURN   	0 1
        // 6	[13]	ADD      	3 3 4
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[5], LuaValue::Integer(99));
            assert_eq!(l.stack.stack[6], LuaValue::Integer(11));
        }));
        // 7	[13]	RETURN   	0 1
        expect_closure.push(Box::new(|l: &LuaState| {
            assert_eq!(l.stack.stack[5], LuaValue::Integer(99));
        }));

        let mut expect_fun = |l: &LuaState| {
            let func = &mut expect_closure[expect_index];
            func(l);
            expect_index += 1;
        };
        l.internal_call(0, &mut Some(&mut expect_fun));
        assert_eq!(expect_index, 12);
    }
}
