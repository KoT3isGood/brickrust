

pub const DEFAULT_ALIGNMENT: u32 = 0u32;
use brickrust_macros::lookup;

pub fn malloc( count: usize, alignment: u32 ) -> *mut ()
{
    panic!("malloc");
}

pub fn realloc( original: *mut(), count: usize, alignment: u32 ) -> *mut ()
{
    panic!("realloc");

}
pub fn free( original: *mut() ) -> bool
{
    panic!("free");

}
pub fn get_allocation_size( original: *mut(), size_out: *mut usize ) -> bool
{
    panic!("get_allocation_size");
}
