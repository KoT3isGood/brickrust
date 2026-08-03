

pub const DEFAULT_ALIGNMENT: u32 = 0u32;
use brickrust_macros::lookup;

unsafe extern "C"
{
    pub fn malloc( count: usize ) -> *mut ();
    pub fn realloc( original: *mut(), count: usize ) -> *mut ();
    pub fn free( original: *mut() ) -> bool;
}
