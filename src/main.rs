#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(const_fn)]

extern crate volatile;
extern  crate rlibc;

mod vga_buffer;
use vga_buffer::{Writer, Color};

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}


pub fn print_something() {
    let mut writer = Writer::new(Color::White, Color::Black);

    writer.write_byte(b'H');
    writer.write_string("ello");
}

#[no_mangle]
pub fn _start() -> ! {
    print_something();

    loop {}
}