#![allow(nonstandard_style)]
use core::mem::zeroed;
use crate::{br_print, brickworks_init, patterns::*, set_module_name};
set_module_name!(b"UnrealEd\0");

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

unsafe extern "C" fn brmk_hook_call()
{

}

#[no_mangle]
unsafe extern "C" fn brickworks_binary_lookup( offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8
{
    core::ptr::null()
}

#[no_mangle]
unsafe extern "C" fn brickworks_cpp_lookup( cpp: *const u8 ) -> *const u8
{
    for dll in BRMK_DLLS.dlls.iter()
    {
        let addr = GetProcAddress(*dll, cpp);
        if !addr.is_null() { return addr; }
    }
    core::ptr::null()
}

pub (crate) struct BRMKLookupInfo<const N: usize> {
    pub dll_names: [*const u8; N],
    pub dlls: [*mut (); N],
    pub dll_addresses: [*const u8; N],
    pub dll_sizes: [usize; N],
}

pub (crate) static mut BRMK_DLLS: BRMKLookupInfo<3> = BRMKLookupInfo
{
    dll_names: [
        b"BrickRigsModKitSteam-BrickRigs.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-Core.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-CoreUObject.dll\0".as_ptr(),
    ],
    dlls: unsafe { zeroed() },
    dll_addresses: unsafe { zeroed() },
    dll_sizes: unsafe { zeroed() },
};

unsafe extern "system"
{
    fn LoadLibraryA( lib: *const u8 ) -> *mut ();
    fn GetProcAddress( lib: *const (), proc: *const u8 ) -> *const u8; 
}

lookup!
{
    pub const Malloc: unsafe extern "C" fn ( count: usize, align: u32 ) -> *mut () = 
    LookupInfo::ProcMangled("?Malloc@FMemory@@SAPEAX_KI@Z");
}

#[export_name = "InitializeModule"]
pub unsafe extern "C" fn InitializeModule() -> *mut BrickRustModule
{
    let module = (Malloc.unwrap())(size_of::<BrickRustModule>(), 0) as *mut BrickRustModule;
    (*module).init();
    module
}

#[repr(C)]
#[allow(nonstandard_style)]
pub struct IModuleInterfaceVTable
{
    pub Destroy: unsafe extern "C" fn( this: *mut BrickRustModule ),
    pub StartupModule: unsafe extern "C" fn( this: *mut BrickRustModule ),
    pub PreUnloadCallback: unsafe extern "C" fn( this: *mut BrickRustModule ),
    pub PostLoadCallback: unsafe extern "C" fn( this: *mut BrickRustModule ),
    pub ShutdownModule: unsafe extern "C" fn( this: *mut BrickRustModule ),
    pub SupportsDynamicReloading: unsafe extern "C" fn( this: *mut BrickRustModule ) -> bool,
    pub SupportsAutomaticShutdown: unsafe extern "C" fn( this: *mut BrickRustModule ) -> bool,
    pub IsGameModule: unsafe extern "C" fn( this: *mut BrickRustModule ) -> bool,
}

#[repr(C)]
pub struct BrickRustModule
{
    pub vtable: *const IModuleInterfaceVTable,
}

impl BrickRustModule
{

    unsafe extern "C" fn destroy( _this: *mut BrickRustModule )
    {

    }

    unsafe extern "C" fn startup_module( _this: *mut BrickRustModule )
    {
        brickworks_init();

    }

    unsafe extern "C" fn pre_unload_callback( _this: *mut BrickRustModule )
    {

    }

    unsafe extern "C" fn post_load_callback( _this: *mut BrickRustModule )
    {

    }

    unsafe extern "C" fn shutdown( _this: *mut BrickRustModule )
    {

    }
    unsafe extern "C" fn supports_dynamic_reloading( _this: *mut BrickRustModule ) -> bool
    {
        return false;
    }
    unsafe extern "C" fn supports_automatic_shutdown( _this: *mut BrickRustModule ) -> bool
    {
        return false;
    }
    unsafe extern "C" fn is_game_module( _this: *mut BrickRustModule ) -> bool
    {
        return false;
    }

    const fn vtable() -> *const IModuleInterfaceVTable
    {
        const VTABLE: IModuleInterfaceVTable = IModuleInterfaceVTable {
            Destroy: BrickRustModule::destroy,
            StartupModule: BrickRustModule::startup_module,
            PreUnloadCallback: BrickRustModule::pre_unload_callback,
            PostLoadCallback: BrickRustModule::post_load_callback,
            ShutdownModule: BrickRustModule::shutdown,
            SupportsDynamicReloading: BrickRustModule::supports_dynamic_reloading,
            SupportsAutomaticShutdown: BrickRustModule::supports_automatic_shutdown,
            IsGameModule: BrickRustModule::is_game_module,
        };
        return &VTABLE;

    }
    fn init(&mut self)
    {
        *self = BrickRustModule 
        {
            vtable: BrickRustModule::vtable()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_hook_internal( _f: *const (), _new_fn: *const() ) -> *const ()
{
    todo!()
}

use std::backtrace::Backtrace;
use std::panic;

#[no_mangle]
unsafe extern "system" fn DllMain(
    _hinstance: HINSTANCE,
    reason: DWORD,
    _reserved: LPVOID,
) -> BOOL
{
    match reason {
        DLL_PROCESS_ATTACH => {
            for (i, name) in BRMK_DLLS.dll_names.iter().enumerate()
            {
                BRMK_DLLS.dlls[i] = LoadLibraryA(name.clone());
            }
            do_lookup();
        }
        _ =>
        {

        }
        
    }
    1
}
