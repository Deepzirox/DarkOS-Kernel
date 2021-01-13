// BUFFER HEIGHT AND WIDTH OF BUFFER
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive traits
#[repr(u8)] // accept u8
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
}// ColorChar : Assign the char a color and background state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorChar(pub u8);

// implementation for ColorChar
impl ColorChar {
        //Get new color from enum
        pub fn new_color(foreground: Color, background: Color) -> ColorChar{
                ColorChar((background as u8) << 4 | (foreground as u8))
        }
        // fill VGA screen with a color
        pub fn fill_background(background: u8){
            let screen_size = BUFFER_HEIGHT * BUFFER_WIDTH;
            let ptr = 0xb8000 as *mut u8;
            let val = (background as u8) << 4 | (background as u8);
            for pos in 0..screen_size {
                unsafe {
                    *ptr.offset(pos as isize * 2 + 1) = val;
                }
            }
        }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// Screenchar contains the color and the ascii character itself
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
// Buffer:  contains chars, the layout of VGA buffer
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
// WriteVGA: contains the line handler and the address of the buffer
pub struct WriteVGA{
        pub color: ColorChar,
        pub line: i32,
        pub col: i32,
        pub vga_buff: &'static mut Buffer // to convert to ptr of Buffer
}

// implementations of WriteVGA
 impl  WriteVGA {
        pub fn putchar(&mut self, ch: char){
            match ch {
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                },
                _ => {
                    if self.col == (BUFFER_WIDTH - 1) as i32 {
                        self.line += 1; self.col = 1;
                    }
                    self.vga_buff.chars[self.line as usize][self.col as usize] = ScreenChar {
                        ascii_character: ch as u8,
                        color_code: self.color
                    };
                    self.col += 1;
                 }
            }
        }
        pub fn print(&mut self, buff: &str) {
            for ch in buff.chars() {
                self.putchar(ch);
            }
        }
 }
 // implementation for formating
 use core::fmt;
impl fmt::Write for WriteVGA {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}
