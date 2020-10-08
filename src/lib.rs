#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]
pub mod api;
pub mod chunk;
pub mod state;
#[macro_use]
pub mod vm;
pub(crate) mod stdlib;
