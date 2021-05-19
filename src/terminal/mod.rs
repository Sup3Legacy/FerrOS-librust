extern crate num;

#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive)]
pub enum Color {
    Black = 0x0_u8,
    Blue = 0x1_u8,
    Green = 0x2_u8,
    Cyan = 0x3_u8,
    Red = 0x4_u8,
    Magenta = 0x5_u8,
    Yellow = 0x6_u8,
    LightGray = 0x7_u8,
    DarkGray = 0x8_u8,
    LightBlue = 0x9_u8,
    LightGreen = 0xA_u8,
    LightCyan = 0xB_u8,
    LightRed = 0xC_u8,
    LightMagenta = 0xD_u8,
    LightYellow = 0xE_u8,
    White = 0xF_u8,
}

impl Color {
    pub fn from_u8(n: u8) -> Self {
        num::FromPrimitive::from_u8(n).unwrap()
    }
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

static mut FG_COLOR: Color = Color::White;
static mut BG_COLOR: Color = Color::Black;

pub fn get_fg() -> u8 {
    unsafe { FG_COLOR.to_u8() }
}

pub fn get_bg() -> u8 {
    unsafe { BG_COLOR.to_u8() }
}

pub fn set_fg(n: u8) {
    unsafe { FG_COLOR = Color::from_u8(n) }
}

pub fn set_bg(n: u8) {
    unsafe { BG_COLOR = Color::from_u8(n) }
}
