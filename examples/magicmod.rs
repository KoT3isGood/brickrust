//! This is an implementation of https://github.com/Redacted00/BR_MagicMod with this mod tool
//!
use brickrust::bp_function;
use brickworks::set_module_name;
use brickworks::br_print;
use brickworks::modinfo::ModInfo;
use brickrust::br;
use brickrust;
use brickrust::ue::blueprint::BlueprintFunction;
use inventory::*;
set_module_name!(b"magicmod\0");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"Redacted's magic mod reimplementation\0".as_ptr(), 
        description: b"\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.2\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

bp_function! ( BeginPlay |obj, stack, result|
{
    br_print!("Small prank");
});

unsafe fn engine_init()
{
    brickrust::warn_version_mismatch!();
}

#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;

    brickrust::init();
    brickrust::hook_post_engine_init(engine_init);
}
