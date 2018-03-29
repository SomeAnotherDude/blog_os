#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]


extern crate x86_64;
extern crate volatile;
extern crate spin;
extern crate rlibc;
#[macro_use]
extern crate lazy_static;


#[macro_use]
mod vga_buffer;
mod interrupts;



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
    interrupts::load_idt();

    for i in 0..30 {
        println!("{}", i);
    }

    print!("{}", 0);
    for i in 1..30 {
        print!(" {}", i);
    }
    println!();

    x86_64::instructions::interrupts::int3();

    print!("{} ", 1);
    for i in 2..20 {
        print!(" {}", i);
    }
    println!();



    loop {}
}
