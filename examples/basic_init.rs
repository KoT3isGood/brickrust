use brickworks::modinfo::ModInfo;
use brickworks::{br_print, set_module_name};
use brickrust::ue::coreuobject::*;
use brickrust::ue::fname;
set_module_name!("basic init");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust basic init\0".as_ptr(), 
        description: b"This is a mod which makes every single work as RC\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.1\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

/**
 * this function is called during the initialization stage of brickworks
 * 
 * call `brickrust::init` to allow usage of brickrust
 * */

static mut INITED: bool = false;

#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    if INITED { return; }
    brickrust::init();
    brickrust::hook_construct_uobject(ue_object_init);
    INITED = false;
}

pub unsafe fn ue_object_init( obj: *mut UObjectBase )
{
    br_print!("{:#?}", (*obj).name_private)
}

pub fn frame()
{
}

pub fn deinit()
{
}
