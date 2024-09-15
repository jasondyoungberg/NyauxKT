use crate::utils;
use core::fmt;
use spin::mutex::Mutex;
struct serial {}
impl serial {
    pub fn serial_putc(&self, w: char) {
        unsafe {
            utils::write_to_portu8(0x3F8, w as u8);
        }
    }
}

impl fmt::Write for serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for i in s.chars() {
            self.serial_putc(i);
        }
        Ok(())
    }
}
static SERIAL: Mutex<serial> = Mutex::new(serial {});

unsafe impl Send for serial {}
#[doc(hidden)]
pub fn serial_print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        core::arch::asm!("cli");
    }

    SERIAL.lock().write_fmt(args).unwrap();
    unsafe {
        core::arch::asm!("sti");
    }
}
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::drivers::serial::serial_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
