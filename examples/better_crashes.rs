#![allow(static_mut_refs)]
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

unsafe fn engine_init()
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
}
