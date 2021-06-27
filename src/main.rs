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

fn asm_add(a: isize, b: isize) -> isize {
    let c: isize;
    unsafe {
        asm!(
            ".byte 0x48, 0x01, 0xf2 \n\t /* {1}, {2} */",
            ".byte 0x48, 0x89, 0xd7 \n\t /* {0} */",
            // "add {1}, {2}",
            // "mov {0}, {1}",
            out(reg) c,
            in(reg) a,
            in(reg) b,
        );
    }
    c
}
