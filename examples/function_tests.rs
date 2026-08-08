//! This example shows the usage of static functions across the game

#![allow(static_mut_refs)]
use core::slice;
use std::intrinsics::copy_nonoverlapping;
use std::ptr::null_mut;

use brickrust::ue::fmath::*;
use brickrust::ue::fname::FName;
use brickrust::ue::fstring::FString;
use brickworks::set_module_name;
use brickworks::br_print;
use brickworks::modinfo::ModInfo;
use brickrust::ue::tarray::TArray;
use brickrust::br;
use brickrust;


set_module_name!(b"function_tests\0");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"Static function usage\0".as_ptr(), 
        description: b"\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.2\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

unsafe fn test_stuff()
{
    brickrust::warn_version_mismatch!();

    let game_version = br::statics::GetProjectVersion();
    br_print!("Game version: {}", game_version);

    let mut mods = TArray::new();
    br::game::instance::GetEnabledModNames(&mut mods);
    for i in 0..mods.num
    {
        br_print!(": {}", *mods.data.add(i as usize));
    }
}

unsafe fn engine_init()
{
    test_stuff();
}

unsafe fn engine_loadmap()
{

}
#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;

    brickrust::init();
    brickrust::hook_post_engine_init(engine_init);
    brickrust::hook_post_load_map(engine_loadmap);
}
