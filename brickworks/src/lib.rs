//!
//! This crate is a basic mod loader for Brick Rigs, made to load dynamic libraries.
//! Provides basic logging and signature finding for mods.
//!
//! # Folder structure
//! This is required folder structure for windows
//! ```txt
//! BrickRigs.exe
//! ...
//! BrickRigs/
//!     Mods/
//!         YourMod/
//!             Content/
//!                 ...
//!             Yourmod.uplugin
//!             mod.dll
//!     Binaries/
//!         Win64/
//!              BrickRigsSteam-Win64-Shipping.exe
//!              brickworks.dll
//!              xinput1_3.dll
//! brickworks/
//!     yourmod.dll      # enabled mod
//!     _yourmod2.dll    # disabled mod
//! brickworks.txt
//! libgcc_s_seh-1.dll      # shared dependencies
//! libwinpthread-1.dll
//! ```
//!
//! # Mod prerequisites
//!
//! We have few requirements for a mod to get loaded. All the functions below must follow C ABI.
//!
//! ## Loading paths
//! - `brickworks/*.dll`
//! - `BrickRigs/Mods/*/mod.dll`
//! 
//! ## Mandatory functions
//! ```
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_info() -> ModInfo
//! {
//!     ...
//! }
//!
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_init()
//! {
//!     ...
//! }
//! ```
//! 
//!
//! ## Optional functions
//! ```
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn mod_deinit()
//! {
//!     ...
//! }
//! ```
//!


#![allow(static_mut_refs)]
pub mod win32;
pub mod logger;
pub mod print;
pub mod modinfo;
pub mod patterns;
pub mod hookmgr;

use libloading::*;
use std::fs;
use std::io;
use std::collections::HashMap;
use core::ffi::CStr;
use core::ffi::c_char;
use std::path::PathBuf;

use std::backtrace::Backtrace;
use std::panic;

static mut MODS: Option<HashMap<String, Library>> = None;

/**
 * Scans for mods and initializes them.
 * 
 * Note: This function must be run before the engine initializes.
 * */
#[no_mangle]
unsafe extern "C" fn brickworks_init() {
    static mut INITED: bool = false;
    if INITED {return}
    INITED = true;

    logger::init();
    panic::set_hook(Box::new(|info| {
        let bt = Backtrace::force_capture();
        br_print!("Panic: {}", info);
        br_print!("Backtrace:\n{}", bt);
    }));
    hookmgr::init();
    MODS = Some(HashMap::new());

    load_mods();

    /*
     * print this thing
     * */
    for m in MODS.as_mut().unwrap().iter()
    {
        use modinfo::ModInfo;
        let mod_info = m.1.get(b"mod_info\0");
        if mod_info.is_err()
        {
            br_print!("Failed to find symbol \"mod_info\"");
            br_print!("Please check for the presence of \"mod_info\"");
            br_print!("Skipped: {}", m.0 );
            continue;
        }

        let mod_init = m.1.get(b"mod_init\0");
        if mod_init.is_err()
        {
            br_print!("Failed to find symbol \"mod_init\"");
            br_print!("Please check for the presence of \"mod_info\"");
            br_print!("Skipped: {}", m.0 );
            continue;
        }
        let mod_info: Symbol<extern "C" fn () -> ModInfo> = mod_info.unwrap();
        let mod_init: Symbol<extern "C" fn ()> = mod_init.unwrap();
        let modinfo = mod_info();

        let name = CStr::from_ptr(modinfo.name as *const c_char);
        let description = CStr::from_ptr(modinfo.description as *const c_char);
        let version = CStr::from_ptr(modinfo.version as *const c_char);
        let game_version = CStr::from_ptr(modinfo.game_version as *const c_char);
        let author = CStr::from_ptr(modinfo.authors as *const c_char);
        br_print!("   {}", name.to_str().unwrap() );
        br_print!("   {}", description.to_str().unwrap() );
        br_print!("   Version: {}", version.to_str().unwrap() );
        br_print!("   Brick Rigs: {}", game_version.to_str().unwrap() );
        br_print!("   Author: {}", author.to_str().unwrap() );

        /* because pointers are per-dll we need to init them */
        mod_init();
        br_print!("Mod initialized: {}", name.to_str().unwrap() );
    }

}

#[no_mangle]
unsafe extern "C" fn brickworks_deinit() {
    MODS = None;
    logger::deinit();
}
#[no_mangle]
unsafe extern "C" fn BrickRust_print( str: *const u8 )
{
    logger::brickworks_puts(
        b"brickworks\0".as_ptr(),
        str
        );
}

unsafe fn load_mods()
{
    let mods = fs::read_dir("brickworks");
    if let Ok(mods) = mods
    {
        let entries = mods;
        for entry in entries
        {
            if entry.is_err() { continue }
            let entry = entry.unwrap();
            load_mod(&entry.path());
        }
    }

    let brmods = fs::read_dir("BrickRigs/Mods");
    if let Ok(mods) = brmods
    {
        let entries = mods;
        for entry in entries
        {
            if entry.is_err() { continue }
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir()
            {
                let dll_path = path.join("mod.dll");
                if dll_path.exists()
                {
                    load_mod(&dll_path);
                }
            }
        }
    }

}

unsafe fn load_mod( entry: &PathBuf)
{
        let filename = entry.file_name();
        let filenamestr = filename.unwrap().to_str().unwrap();
        if filenamestr.starts_with("_")
        {
            br_print!("{}: DISABLED",filenamestr.trim_start_matches("_"));
            return;
        }
        else 
        {
            br_print!("{}: ENABLED",filenamestr);
        }
        let lib = Library::new(entry.as_path());
        if lib.is_err()
        {
            br_print!("Failed to load {}: {} ({})", entry.as_path().display(), lib.err().unwrap(), io::Error::last_os_error()); 
            return;
        }

        MODS.as_mut().unwrap().insert(entry.to_str().unwrap().to_string(), lib.unwrap());
}
