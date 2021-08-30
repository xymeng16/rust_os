use crate::serial_println;
use bootloader::boot_info::{FrameBufferInfo, PixelFormat};
use core::{
    fmt::{self, Write},
    mem, ptr,
};
use font8x8::UnicodeFonts;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    // pub static ref WRITER: [u8; mem::size_of::<Writer>()] = [0; mem::size_of::<Writer>()];
    pub static ref WRITER: Mutex<[u8; mem::size_of::<Writer>()]> = {
        let val = Mutex::new([0 as u8; 80]);
        val
    };
}

#[macro_export]
macro_rules! GLOBAL_WRITER {
    () => {
        *(WRITER.lock().as_ptr() as *const Writer)
    };
}

#[macro_export]
macro_rules! GLOBAL_WRITER_MUT {
    () => {
        *(WRITER.lock().as_ptr() as *mut Writer)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (unsafe { $crate::vga_buffer::_print(format_args!($($arg)*))});
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub unsafe fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        GLOBAL_WRITER_MUT!()
            .write_fmt(args)
            .expect("Printing to VGA failed");
    });
}

pub fn init_global_writer(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
    unsafe {
        GLOBAL_WRITER_MUT!() = Writer::new(framebuffer, info);
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    #[allow(dead_code)]
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const LINE_SPACING: usize = 0;

pub struct Writer {
    framebuffer: &'static mut [u8],
    pub info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl Writer {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writter = Writer {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        writter.clear();
        writter
    }

    fn newline(&mut self) {
        self.y_pos += 8 + LINE_SPACING;
        self.carriage_return()
    }

    #[allow(dead_code)]
    fn add_vspace(&mut self, space: usize) {
        self.y_pos += space;
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    /// Erases all text on the screen.
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width() {
                    self.newline();
                }
                if self.y_pos >= (self.height() - 8) {
                    self.clear();
                }
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8; 8]) {
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let alpha = if *byte & (1 << bit) == 0 { 0 } else { 255 };
                self.write_pixel(self.x_pos + x, self.y_pos + y, alpha);
            }
        }
        self.x_pos += 8;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity / 2, 0],
            PixelFormat::BGR => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => {
                todo!();
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for Writer {}

unsafe impl Sync for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
