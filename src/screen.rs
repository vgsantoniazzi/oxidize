use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// Define the screen size as
// 80 columns x per 25 lines
// We are managing the VGA buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Public interface screen
// Use this struct and implementation
// to control screen data and manipulate
// the VGA Buffer -- 0xb8000 address
pub struct Screen {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

// Use this lazy_static! to control a Singleton Mutex.
// Since this is not trivial to do with Rust, neither
// this piece of software, we are using some workarounds
// to accomplish this.
lazy_static! {
    pub static ref WRITER: Mutex<Screen> = Mutex::new(Screen {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Screen {
    // To implement macro `write!`
    #[allow(dead_code)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }

    // Write a string to the buffer.
    // This is a wrapper to write a string
    // instead of creating the for in a lot
    // of different places.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    // We need to manipulate the memory address
    // in order to display the message correctly
    // we need to understand if the next line is
    // a new line char (\n) or an ordinary byte.
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_new_line_to_buffer(),
            byte => self.write_new_byte_to_buffer(byte),
        }
    }

    // Internal function to write any new byte to
    // the VGA address. If the line reaches the
    // width we need to add a new line -- offset the
    // buffer and continue to writing things there.
    fn write_new_byte_to_buffer(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.write_new_line_to_buffer();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col] = ScreenChar {
            ascii_character: byte,
            color_code,
        };

        self.column_position += 1;
    }

    // To write a new line to the buffer
    // we need to move the data inside the
    // buffer one height up -- or one
    // row above. This will help us to
    // clear messages when the rows reaches
    // the limit and, we will lost the message.
    fn write_new_line_to_buffer(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // This function help us to add blank
    // Chars to the end of each row, in
    // order to keep the colors nice
    // and does not show things broken
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

// Each Color represents the VGA color
// https://en.wikipedia.org/wiki/VGA_text_mode
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// Structure to manage color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

// Return the correct color code based on the
// foregroung and the background
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Each of ScreenChar represents one char
// and the correct color code to print in
// the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// Map the VGA buffer to this struct. We are able
// to write an char (with background) and
// set the line and column size (defined on the top
// of the file)
#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
