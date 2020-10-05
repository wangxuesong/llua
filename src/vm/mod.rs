mod instruction;
mod lua_vm;
pub(crate) mod opcodes;
mod upvalue;

pub use instruction::Instruction;
pub use lua_vm::read_chunk;
