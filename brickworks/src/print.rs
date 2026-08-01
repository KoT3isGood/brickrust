use core::fmt::{self, Write};
use core::ptr;
use core::slice;
extern "C"
{
    fn malloc( size: usize ) -> *mut ();
    fn realloc( ptr: *mut (), size: usize ) -> *mut ();
    fn free( ptr: *mut () );
}

pub struct Buffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}
impl Buffer {
    pub fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn grow(&mut self, additional: usize) -> Result<(), ()> {
        let required = self.len + additional;

        if required <= self.cap {
            return Ok(());
        }

        let new_cap = if self.cap == 0 {
            required.max(64)
        } else {
            (self.cap * 2).max(required)
        };

        unsafe {
            let new_ptr = if self.ptr.is_null() {
                malloc(new_cap)
            } else {
                realloc(self.ptr as *mut _, new_cap)
            } as *mut u8;

            if new_ptr.is_null() {
                return Err(());
            }

            self.ptr = new_ptr;
            self.cap = new_cap;
        }

        Ok(())
    }
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.grow(s.len()).map_err(|_| fmt::Error)?;

        unsafe {
            let dst = self.ptr.add(self.len);
            ptr::copy_nonoverlapping(s.as_ptr(), dst, s.len());
        }

        self.len += s.len();
        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free(self.ptr as *mut _);
            }
        }
    }
}


unsafe extern "C"
{
    pub fn BrickRust_print( str: *const u8 );
}

#[macro_export]
macro_rules! set_module_name {
    ($name: expr) => {
        #[allow(non_snake_case)]
        pub unsafe fn BrickRust_print( str: *const u8 )
        {
            use brickworks::logger;
            logger::brickworks_puts(
                $name.as_ptr(),
                str
                );
        }
        
    };
}

#[macro_export]
macro_rules! br_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::print::Buffer;

        let mut buf = Buffer::new();
        let _ = write!(&mut buf, "{}\0", core::format_args!($($arg)*));

        unsafe {
            BrickRust_print(buf.ptr as *const u8);
        }
    }};
}
