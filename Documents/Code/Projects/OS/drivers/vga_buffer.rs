/// Represents a color in the VGA text mode. Each color is represented by a 4-bit value (0-15).
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

/// A ColorCode is a combination of a foreground (4 bits) and background color (4 bits), represented as a single byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

/// Computes a ColorCode from a background and foreground color.
/// 
/// A ColorCode is a combination of a foreground (4 bits) and background color (4 bits), represented as a single byte.
/// 
/// # Example
/// Background: Black (0), Foreground: White (15)
/// ColorCode = (0 << 4) | 15
/// = 00000000 | 00001111
/// = 00001111
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A screen character in VGA text mode, consisting of an ASCII character and a ColorCode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;    // The height of the VGA text buffer in rows.
const BUFFER_WIDTH: usize = 80;   // The width of the VGA text buffer in columns.

/// A buffer representing the VGA text mode screen, consisting of a 2D array of ScreenChar.
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}