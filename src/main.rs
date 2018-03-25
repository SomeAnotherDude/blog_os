#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(const_fn)]

extern crate volatile;

mod vga_buffer;


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


use core::ptr;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub fn _start() -> ! {
    let vga_buffer = 0xb8000 as *const u8 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            let i = i as isize;
            ptr::write(vga_buffer.offset(i * 2), byte);
            ptr::write(vga_buffer.offset(i * 2 + 1), 0xb);
        }
    }

    loop {}
}