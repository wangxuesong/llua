#[macro_use]
mod macros {
    #[cfg(debug_assertions)]
    #[macro_export]
    macro_rules! debug {
        ($x:expr) => {
            dbg!($x)
        };
    }

    #[cfg(not(debug_assertions))]
    #[macro_export]
    macro_rules! debug {
        ($x:expr) => {
            std::convert::identity($x)
        };
    }
}

mod instruction;
mod lua_vm;
pub mod opcodes;
mod upvalue;

pub use lua_vm::read_chunk;
pub use instruction::Instruction;
