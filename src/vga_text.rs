use core::fmt::{Write};

const BUFFER_WIDTH: u16 = 80;
const BUFFER_HEIGHT: u16 = 25;

#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// A VGA text-mode color
pub enum Color {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	LightGray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xa,
	LightCyan = 0xb,
	LightRed = 0xc,
	Pink = 0xd,
	Yellow = 0xe,
	White = 0xf,
}

#[repr(transparent)]
/// Color codes are made of a 8 bit background color followed by a 8 bit foreground color
struct ColorCode(u8);

impl ColorCode {
	fn new(fg: Color, bg: Color) -> Self {
		ColorCode((fg as u8) | ((bg as u8) << 4))
	}
}

#[repr(transparent)]
/// a character is made of a char (almost ASCII) and a color code
pub struct Char(u16);

impl Char {
	fn new_from_color_code(c: char, col: ColorCode) -> Self {
		Char((c as u16 | ((col.0 as u16) << 8)) as u16)
	}
	pub fn new(c: char, fg: Color, bg: Color) -> Self {
		Self::new_from_color_code(c, ColorCode::new(fg, bg))
	}
	/// Get the actual `char` this `Char` represents
	pub fn char(&self) -> char {
		char::from_u32((&self.0 & (0xff)) as u32).expect("cannot convert char")
	}
}

/// get the position in the buffer of a char at coordinates (x, y)
fn get_offset(x: u16, y: u16) -> u32 {
	(x + y * BUFFER_WIDTH).into()
}

/// write a `Char` to the screen
pub fn putchar(c: Char, (x, y): (u16, u16)) {
	let pos = get_offset(x, y);
	let buffer_ptr = (0xb8000 + 2 * pos) as *mut u32;
	unsafe { *buffer_ptr = c.0 as u32 }
}

/// Allows to write a whole string by keeping track of the position of characters.
/// It supports line wrapping.
/// Must be used with `core::fmt::Write` (this allows formatting)
pub struct VgaWriter {
	pos: (u16, u16),
	fg: Color,
	bg: Color
}

impl VgaWriter {
	pub fn new(fg: Color, bg: Color) -> VgaWriter {
		VgaWriter {
			pos: (0,0),
			fg,
			bg
		}
	}
	
	pub fn default() -> VgaWriter {
		Self::new(Color::White, Color::Black)
	}
	
	fn wrap(&mut self) {
		self.pos.1 += 1;
		self.pos.0 = 0
	}
	
	fn newline(&mut self) {
		todo!()
	}
}

impl Write for VgaWriter {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for c in s.chars() {
			match c {
				'\n' => self.wrap(),
				c => {
					let ch = Char::new(c, self.fg, self.bg);
					putchar(ch, self.pos);
					
					if self.pos.0 >= BUFFER_WIDTH - 1{
						if self.pos.1 >= BUFFER_HEIGHT - 1{
							self.newline();
						}
						self.wrap();
					} else {
						self.pos.0 += 1;
					}
				}
			}
		}
		Ok(())
	}
}
