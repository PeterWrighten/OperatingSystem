#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;

mod sbi;
mod loader;
mod config;
mod task;

global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("link_app.S"));