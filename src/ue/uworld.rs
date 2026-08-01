#![allow(nonstandard_style)]
pub static mut GWORLD_PTR: *const *mut () = core::ptr::null();

pub unsafe fn GWorld() -> *mut ()
{
    return *GWORLD_PTR;
}
