// #![no_std]
// #![no_main]
// #![feature(lang_items)]
// #![feature(alloc_error_handler)]
// #![feature(panic_info_message)]

// use ckb_std::default_alloc;

// ckb_std::entry!(program_entry);
// default_alloc!();

#![feature(asm)]

fn main() {
    let a = 234;
    let b = 333;
    println!("add({}, {}) => {}", a, b, asm_add(a, b));
}

#[no_mangle]
pub extern "C" fn asm_add(a: i32, b: i32) -> i32 {
    let c: i32;
    unsafe {
        asm!(
            ".byte 0x90",
            "add {1}, {2}",
            "mov {0}, {1}",
            out(reg) c,
            in(reg) a,
            in(reg) b,
        );
    }
    c
}
