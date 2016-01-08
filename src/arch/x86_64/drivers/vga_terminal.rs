use core::ptr::Unique;
use core::fmt;
use spin::Mutex;

pub static WRITER: Mutex<BufferWriter> = Mutex::new(BufferWriter {
    column_position: 0, row_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            $crate::arch::x86_64::drivers::vga_terminal::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}


#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
pub struct ColorCode(u8);
impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BufferCharacter {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

struct Buffer {
    characters: [[BufferCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct BufferWriter {
    column_position: usize, row_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl BufferWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); // wrap lines
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer().characters[row][col] = BufferCharacter {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.column_position += 1;
            }
        }
    }

    pub fn write_byte_at_pos(&mut self, byte: u8, row: usize, col: usize) {
        self.column_position = col; self.row_position = row;

        self.buffer().characters[row][col] = BufferCharacter {
            ascii_character: byte,
            color_code: self.color_code,
        }
    }

    pub fn write_str_at_pos(&mut self, string: &str, row: usize, col: usize) {
        let mut col_accumulator: usize = col;
        for byte in string.bytes() {
            self.write_byte_at_pos(byte, row, col_accumulator);
            col_accumulator += 1;
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;

        if self.row_position >= BUFFER_HEIGHT {
            self.scroll();
        }
    }

    fn clear_row(&mut self, row: usize) {
        self.buffer().characters[row] = [BufferCharacter {ascii_character: b' ', color_code: self.color_code}; BUFFER_WIDTH];
    }

    pub fn clear_screen(&mut self) {
        self.buffer().characters = [[BufferCharacter {ascii_character: b' ', color_code: self.color_code}; BUFFER_WIDTH]; BUFFER_HEIGHT];
    }

    fn scroll(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            let buffer = self.buffer();
            buffer.characters[row] = buffer.characters[row + 1];
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.row_position = BUFFER_HEIGHT - 1;
    }

    pub fn set_color_code(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn get_color_code(&self) -> ColorCode {
        self.color_code
    }
}

impl fmt::Write for BufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
