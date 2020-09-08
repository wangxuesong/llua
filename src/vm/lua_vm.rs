use crate::state::LuaState;
use crate::vm::opcodes::OP_RETURN;
use crate::vm::Instruction;

pub fn lua_vm_execute(l: &mut LuaState, func: &mut Option<&mut dyn FnMut(&LuaState)>) {
    loop {
        let inst = l.fetch();
        if inst.opcode() == OP_RETURN {
            break;
        }
        inst.execute(l);
        match func {
            Some(f) => f(l),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::binary::{Chunk, Prototype};
    use crate::state::{LuaState, LuaValue};
    use crate::vm::lua_vm::lua_vm_execute;
    use std::fs::read;

    fn read_chunk(name: &str) -> Prototype {
        // let name = "foo.out";
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
            LuaValue::Integer(0),
            LuaValue::Integer(1),
            LuaValue::Integer(3),
        ];
        let mut cls = |l: &LuaState| {
            dbg!("hello");
            // assert_eq!(l.stack.stack.len(), 2);
            assert_eq!(l.stack.stack[0], expect[expect_index]);
            expect_index += 1;
        };
        lua_vm_execute(&mut l, &mut Some(&mut cls))
    }
}
