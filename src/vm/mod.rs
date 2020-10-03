mod instruction;
mod lua_vm;
pub(crate) mod opcodes;

pub use instruction::Instruction;
pub use lua_vm::{lua_vm_execute, read_chunk};
