#![allow(unused_variables)]
use brickworks::patterns::*;
use brickworks::br_print;
use brickworks::set_module_name;
set_module_name!(b"stub\0");

#[no_mangle]
pub unsafe extern "C" fn brickworks_binary_lookup( offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8
{
    core::ptr::null()

}

#[no_mangle]
pub unsafe extern "C" fn brickworks_cpp_lookup( cpp: *const u8 ) -> *const u8
{
    core::ptr::null()
}

#[no_mangle]
unsafe extern "C" fn brickworks_binary_dll_lookup( dll: *const u8, offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8
{
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_hook_internal( f: *const (), new_fn: *const() ) -> *const ()
{
    br_print!("why?");
    core::ptr::null()
}
