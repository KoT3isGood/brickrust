#![allow(nonstandard_style)]
use core::mem::zeroed;
use brickworks::{set_module_name, br_print, patterns::*};
use crate::win_universal::brickworks_init;
set_module_name!(b"UnrealEd\0");
use min_hook_rs::*;

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
unsafe extern "C"
{
    pub fn strcmp( l: *const u8, r: *const u8 ) -> i32;
}

#[no_mangle]
unsafe extern "C" fn brickworks_binary_lookup( _offset: isize, _mode: LookupMode, _sign: CSignature ) -> *const u8
{
    unimplemented!()
}

#[no_mangle]
unsafe extern "C" fn brickworks_binary_dll_lookup( dll: *const u8, offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8
{
    for i in 0..BRMK_DLLS.dll_addresses.len()
    {
        if strcmp(BRMK_DLLS.dll_names[i], dll) != 0
        {
            continue;
        }
        let data_len: usize = BRMK_DLLS.dll_sizes[i];
        let data: *const u8 = BRMK_DLLS.dll_addresses[i];
        let addr = lookup_data(data, data_len, sign.clone());
        return lookup_offset(addr, offset, mode);
    }
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
    pub dlls: [HMODULE; N],
    pub dll_addresses: [*const u8; N],
    pub dll_sizes: [usize; N],
}

pub (crate) static mut BRMK_DLLS: BRMKLookupInfo<5> = BRMKLookupInfo
{
    dll_names: [
        b"BrickRigsModKitSteam-BrickRigs.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-Core.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-CoreUObject.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-Engine.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-UnrealEd.dll\0".as_ptr(),
    ],
    dlls: unsafe { zeroed() },
    dll_addresses: unsafe { zeroed() },
    dll_sizes: unsafe { zeroed() },
};

unsafe extern "system"
{
    fn LoadLibraryA( lib: *const u8 ) -> HMODULE;
    fn GetProcAddress( lib: HMODULE, proc: *const u8 ) -> *const u8; 
}

lookup!
{
    pub const Malloc: unsafe extern "C" fn ( count: usize, align: u32 ) -> *mut () = 
    LookupInfo::ProcMangled("?Malloc@FMemory@@SAPEAX_KI@Z");
}

#[export_name = "InitializeModule"]
unsafe extern "C" fn InitializeModule() -> *mut BrickRustModule
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
pub unsafe extern "C" fn brickworks_hook_internal( old_fn: *const (), new_fn: *const() ) -> *const ()
{
    let r = create_hook( old_fn as *mut c_void, new_fn as *mut c_void );
    if r.is_err() { return core::ptr::null(); }
    let f = r.unwrap();

    let r = enable_hook( old_fn as *mut c_void );
    if r.is_err() { return core::ptr::null(); }

    core::mem::transmute(f)
}

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
            for (i, name) in BRMK_DLLS.dll_names.iter().enumerate()
            {
                BRMK_DLLS.dlls[i] = LoadLibraryA(name.clone());
                let module = BRMK_DLLS.dlls[i];
                let process = GetCurrentProcess();
                let mut modinfo: MODULEINFO = zeroed();
                GetModuleInformation(process, module, &mut modinfo, size_of::<MODULEINFO>() as u32);
                BRMK_DLLS.dll_sizes[i] = modinfo.size as usize;
                BRMK_DLLS.dll_addresses[i] = modinfo.base as *const u8;
            }
            // We need it for the symbols. other ones are handled by brickworks+brickrust
            do_lookup();
        }
        _ =>
        {

        }
        
    }
    1
}

unsafe extern "C"
{
    fn fopen( path: *const u8, mode: *const u8 ) -> *mut ();
    fn fclose( stream: *mut () ) -> i32;
    fn fprintf( stream: *mut (), format: *const u8, ... ) -> i32;
    fn fflush( stream: *mut () ) -> i32;
    fn _lock_file( stream: *mut () );
    fn _unlock_file( stream: *mut () );
}
