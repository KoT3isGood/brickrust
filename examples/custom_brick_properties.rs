#![allow(static_mut_refs)]
use brickworks::modinfo::ModInfo;
use brickworks::br_print;

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust custom properties mod\0".as_ptr(), 
        description: b"Adds custom property to each brick\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.1\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

#[no_mangle]
unsafe extern "C" fn init()
{
}

#[no_mangle]
unsafe extern "C" fn frame()
{
}

pub fn deinit()
{
}
