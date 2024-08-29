
use flanterm_bindings;
use spin::mutex::Mutex;
use core::convert::Into;
use core::convert::TryInto;
use core::fmt;
use core::mem::size_of_val;
use core::option::Option::None;
use limine::framebuffer::Framebuffer;

use crate::TERM;
const BG: u32 = 0x181a21;
const FG: u32 = 0xe3e3de;
pub struct NyauxTerm {
    ctx: Option<*mut flanterm_bindings::flanterm_context>
}
unsafe impl Send for NyauxTerm {}
impl NyauxTerm {
    pub fn new_none() -> Mutex<Self>
    {
        Mutex::new(NyauxTerm {
            ctx: None
        })
    }
    pub fn init_basic(&mut self, f: Framebuffer)
    {
        unsafe {
            let ctx = flanterm_bindings::flanterm_fb_init(None, None, f.addr() as *mut u32, f.width().try_into().unwrap(), f.height().try_into().unwrap(), f.pitch().try_into().unwrap(), f.red_mask_size().into(), f.red_mask_shift().into(), f.green_mask_size().into(), f.green_mask_shift().into(), f.blue_mask_size().into(), f.blue_mask_shift().into(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), &mut BG as *mut u32, &mut FG as *mut u32, core::ptr::null_mut(),core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, 1, 0, 0, 0);
            self.ctx = Some(ctx);
        }   
        
    }
    pub fn write_string(&mut self, s: &str)
    {
        unsafe {
            flanterm_bindings::flanterm_write(self.ctx.unwrap(), s.as_ptr() as *const i8, size_of_val(s));
        }
        
    }
    pub fn clear_screen(&mut self, col: u32)
    {
        unsafe {
            // cursed
            ((*self.ctx.unwrap()).set_text_bg_rgb.unwrap())(self.ctx.unwrap(), col as u32);
            ((*self.ctx.unwrap()).clear.unwrap())(self.ctx.unwrap(), true);
        }
    }
}
impl fmt::Write for NyauxTerm
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::utils::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    TERM.lock().write_fmt(args).unwrap();
}
#[derive(Debug)]
pub enum KTError
{
    NotImplmented,
    OperationFailed,
    OutOfMemory
}