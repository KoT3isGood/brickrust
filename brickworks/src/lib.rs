#![allow(static_mut_refs)]
//! 
//! brickworks is a mod manager for all brickrust mods
//!
//! it is a mod manager which loads all the mods you need
//!
//! we do not use no_std as it is kinda high-level thing
//!
pub mod win32;
pub mod logger;
pub mod print;
pub mod modinfo;
pub mod patterns;

use libloading::*;
use std::fs;
use std::io;
use std::collections::HashMap;
use std::ffi::OsString;
use core::ffi::CStr;
use core::ffi::c_char;


static mut MODS: Option<HashMap<OsString, Library>> = None;


/**
 * must be called by the loader when it is loaded
 * DllMain for windows
 * .init_array for linux
 * */
#[no_mangle]
unsafe extern "C" fn brickworks_init() {
    logger::init();
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
        let f_ = m.1.get(b"mod_init\0");
        let f: Symbol<extern "C" fn ()> = f_.unwrap();
        f();
    }
}

#[no_mangle]
unsafe extern "C" fn brickworks_deinit() {
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
