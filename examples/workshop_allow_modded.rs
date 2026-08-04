//! This mod introduces a function override which makes the game think there are no modded bricks in
//! a vehicle

#![allow(static_mut_refs)]
use brickrust::ue::coreuobject::UObject;
use brickworks::hookmgr::hook;
use brickworks::modinfo::ModInfo;
use brickworks::patterns::*;
use brickrust::br;
use brickrust;
use brickrust_macros::sig;

use core::mem::zeroed;

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"Modded bricks in workshop\0".as_ptr(), 
        description: b"This mod allows usage of modded bricks while uploading vehicles in workshop.\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.2\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}



unsafe extern "C" fn my_is_asset_modded( param: *mut UObject ) -> bool
{
    false
}


#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    static mut INITED: bool = false;
    if INITED { return; }

    brickrust::init();

    hook(
        br::statics::IsModdedAsset_ptr.unwrap() as *mut (), 
        my_is_asset_modded as *const ()
    );

    INITED = true;
}
