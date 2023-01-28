#![feature(pointer_byte_offsets)]
#![feature(thread_local)]
#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub use amd64::interrupts;
pub use macros::*;

mod amd64;
mod framebuffer;
mod heap;
mod logger;
