#![allow(nonstandard_style)]

//!
//! A really simple proxy for xinput1_3.dll, which loads brickworks
//!
//! uses assembly boo!
//!

use core::ffi::*;
type BOOL = i32;
type HANDLE = *mut c_void;
type HINSTANCE = HANDLE;
type HMODULE = HANDLE;
type DWORD = u32;
type LPVOID = *mut c_void;
type LPCSTR = *const u8;

const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;


unsafe extern "system"
{
    fn LoadLibraryA( lib: LPCSTR ) -> HMODULE;
    fn GetProcAddress( lib: HMODULE, proc: LPCSTR ) -> usize; 
}
static mut LIB: HMODULE = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn InitializeModule() -> *mut ()
{
    let proc = GetProcAddress(LIB, b"InitializeModule\0".as_ptr());
    let proc: unsafe extern "C" fn() -> *mut () = core::mem::transmute(proc);
    proc()
}

#[no_mangle]
unsafe extern "system" fn DllMain(
    _hinstance: HINSTANCE,
    reason: DWORD,
    _reserved: LPVOID,
) -> BOOL
{
    match reason {
        DLL_PROCESS_ATTACH => {
            LIB = LoadLibraryA(b"brickworks.dll\0".as_ptr());
        }
        DLL_PROCESS_DETACH => {

        }
        _ =>
        {

        }
        
    }
    1
}
