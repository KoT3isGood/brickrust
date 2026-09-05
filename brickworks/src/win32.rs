
use core::{ffi::*, mem::zeroed};
use crate::brickworks_init;
use crate::brickworks_deinit;
use crate::patterns::*;
use min_hook_rs::*;

type BOOL = i32;
type HANDLE = *mut c_void;
type HINSTANCE = HANDLE;
type HMODULE = HANDLE;
type DWORD = u32;
type LPVOID = *mut c_void;
type LPCSTR = *const u8;


const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;

#[repr(C)]
struct MODULEINFO
{
    base: *mut u8,
    size: DWORD,
    entry: *mut (),
}

unsafe extern "system"
{
    fn GetModuleHandleA( module: LPCSTR ) -> HMODULE;
    fn GetModuleInformation( process: HANDLE, module: HMODULE, modinfo: *mut MODULEINFO, cb: DWORD ) -> BOOL;
    fn GetCurrentProcess() -> HANDLE;
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
            let module = GetModuleHandleA(core::ptr::null());
            let process = GetCurrentProcess();
            let mut modinfo: MODULEINFO = zeroed();
            GetModuleInformation(process, module, &mut modinfo, size_of::<MODULEINFO>() as u32);
            BASE_ADDRESS = modinfo.base;
            BASE_SIZE = modinfo.size as usize;
            
            
            brickworks_init();
        }
        DLL_PROCESS_DETACH => {
            brickworks_deinit();
        }
        _ =>
        {

        }
        
    }
    1
}

static mut BASE_ADDRESS: *const u8 = core::ptr::null_mut();
static mut BASE_SIZE: usize = 0;

/**
 * returns base address of a application
 * used for signature scanning mainly
 * */
pub unsafe fn get_base_address() -> *const u8
{
    BASE_ADDRESS
}

/**
 * returns base address of a application
 * used for signature scanning mainly
 * */
pub unsafe fn get_base_size() -> usize
{
    BASE_SIZE
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_binary_lookup( offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8
{
    let data_len: usize = get_base_size();
    let data: *const u8 = get_base_address();
    let addr = lookup_data(data, data_len, sign);
    lookup_offset(addr, offset, mode)
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_cpp_lookup( cpp: *const u8 ) -> *const u8
{
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_hook_internal( f: *const (), new_fn: *const() ) -> *const ()
{
    let r = create_hook( f as *mut c_void, new_fn as *mut c_void );
    if r.is_err() { return core::ptr::null(); }
    let f = r.unwrap();

    let r = enable_hook(f as *mut c_void);
    if r.is_err() { return core::ptr::null(); }

    core::mem::transmute(f)
}
