use brickworks::modinfo::ModInfo;
use brickworks::{br_print, set_module_name};
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
 * call `brickrust::init` and `brickrust::hook_ue_init`
 * */
#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    brickrust::init();
    brickrust::hook_ue_init(ue_init);
    br_print!("mod_init");
}

pub unsafe fn ue_init()
{
}

pub fn frame()
{
}

pub fn deinit()
{
}
