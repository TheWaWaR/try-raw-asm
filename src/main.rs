#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(asm)]

use alloc::format;
use ckb_std::{default_alloc, syscalls::debug};

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    let a: i64 = 100;
    let b: i64 = 200;
    debug(format!("    asm add: {} + {} = {}\n", a, b, asm_add(a, b)));
    debug(format!("raw asm add: {} + {} = {}\n", a, b, raw_asm_add(a, b)));
    0
}

fn asm_add(a: i64, b: i64) -> i64 {
    let c: i64;
    unsafe {
        asm!(
            "ADDI x0, x0, 0",
            "ADDI x0, x0, 0",
            "ADDI x0, x0, 0",
            "ADDI x0, x0, 0",
            "ADDI x0, x0, 0",
            "add {0}, {1}, {2}",
            out(reg) c,
            in(reg) a,
            in(reg) b,
        );
    }
    c
}

fn raw_asm_add(a: i64, b: i64) -> i64 {
    let c: i64;
    // $ riscv64-unknown-elf-objdump -d target/riscv64imac-unknown-none-elf/release/deps/try_raw_asm-5cc3a156705d970b > xxx.s
    // In xxx.s
    // 11af4:	00b50633          	add	a2,a0,a1
    unsafe {
        asm!(
            ".byte 0x33, 0x06, 0xb5, 0x00 \n /* {0}, {1}, {2} */",
            out(reg) c,
            in(reg) a,
            in(reg) b,
        );
    }
    c
}
