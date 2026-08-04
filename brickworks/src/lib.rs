//!
//! This crate is a basic mod loader for Brick Rigs, made to load dynamic libraries.
//! Provides basic logging and signature finding for Rust frontends?.
//!
//! # Folder structure
//! 
//! ```txt
//! BrickRigs.exe
//! BrickRigs/
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
//! # Functions provided to a mod
//!
//! Required functions for a mod:
//! - `mod_info` -- returns mod metadata
//! - `mod_init` -- initializes mod
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
use std::ffi::OsString;
use core::ffi::CStr;
use core::ffi::c_char;


static mut MODS: Option<HashMap<OsString, Library>> = None;


/**
 * Scans for mods and initializes them.
 * 
 * Note: This function must be run before the engine initializes.
 * */
#[no_mangle]
unsafe extern "C" fn brickworks_init() {
    logger::init();
    hookmgr::init();
    MODS = Some(HashMap::new());

    load_mods();

    /*
     * print this thing
     * */
    for m in MODS.as_mut().unwrap().iter()
    {
        use modinfo::ModInfo;
        let f_ = m.1.get(b"mod_info\0");
        let f: Symbol<extern "C" fn () -> ModInfo> = f_.unwrap();
        let modinfo = f();

        br_print!("{}: ENABLED",m.0.display());
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
        let mod_init_ = m.1.get(b"mod_init\0");
        let mod_init: Symbol<extern "C" fn ()> = mod_init_.unwrap();
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
    if mods.is_err() {
        br_print!("Failed to find \"brickworks\" folder: {}. It must be in root with BrickRigs.exe. ", mods.err().unwrap()); 
        return;
    }
    let entries = mods.unwrap();

    for entry in entries
    {
        if entry.is_err() { continue }
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let filenamestr = filename.to_str().unwrap();
        if filenamestr.starts_with("_")
        {
            br_print!("{}: DISABLED",filenamestr.trim_start_matches("_"));
            continue;
        }
        let lib = Library::new(entry.path());
        if lib.is_err()
        {
            br_print!("Failed to load {}: {} ({})", entry.path().display(), lib.err().unwrap(), io::Error::last_os_error()); 
            continue;
        }

        MODS.as_mut().unwrap().insert(filename, lib.unwrap());
    }
}
