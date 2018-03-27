#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(const_fn)]

extern crate volatile;
extern crate spin;
extern crate rlibc;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    msg: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    column: u32,
) -> ! {
    println!("PANIC in file {}:{}:{}", file ,line, column);
    println!("{}", msg);
    loop {}
}


#[no_mangle]
pub fn _start() -> ! {
    for i in 0..30 {
        println!("{}", i);
    }

    print!("{}", 0);
    for i in 1..30 {
        print!(" {}", i);
    }
    println!();

    print!("{} ", 1);
    for i in 2..20 {
        print!(" {}", i);
    }
    println!();

    loop {}
}
