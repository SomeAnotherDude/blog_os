use core::fmt;
use volatile::Volatile;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}


struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer {
    col_pos: usize,
    row_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// constructor
    /// cursor position will be at the left-upper corner of screen
    pub fn new(text: Color, background: Color) -> Self {
        Writer {
            col_pos: 0,
            row_pos: 0,
            color_code: ColorCode::new(text, background),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    /// write a single byte to screen
    /// the byte will be considered as an ascii character
    ///
    /// NOTE: only ascii character are supported,
    /// any non-ascii ones will be displayed wrong way
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.newline();
                }

                let row = self.row_pos;
                let col = self.col_pos;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

                self.col_pos += 1;
            }
        }
    }

    /// write a string to screen
    /// printing starts from the current cursor's position
    ///
    /// NOTE: string will be treated as a sequence of bytes,
    /// therefore all non-ascii characters will look... strange
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    /// move cursor to beginning of the next row
    /// if the row is the bottom row then shift all lines up by one row
    /// (first line will be overwritten) and clear the last (bottom) one
    pub fn newline(&mut self) {
        self.col_pos = 0;

        if self.row_pos == BUFFER_HEIGHT - 1 {
            // move all the rows to one line up
            // the first row (upper) is going to be overwritten
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let ch = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(ch);
                }
            }

            // blank the last (bottom) row
            self.clear_rest_of_line();
        }
        else {
            self.row_pos += 1;
        }
    }

    /// overwrites all the rest of the current (self.row_pos)
    /// line (i.e self.col_pos .. BUFFER_WEIGHT) with spaces
    fn clear_rest_of_line(&mut self) {
        for col in self.col_pos .. BUFFER_WIDTH {
            let ch = ScreenChar { ascii_character: b' ', color_code: self.color_code };
            self.buffer.chars[self.row_pos][col].write(ch);
        }
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static!{
    pub static ref VGA_WRITER: Mutex<Writer> = Mutex::new(Writer::new(Color::White, Color::Black));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}



