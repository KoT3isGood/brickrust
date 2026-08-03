use brickrust::ue::tarray::*;
use brickworks::modinfo::ModInfo;
use brickrust::ue::coreuobject::*;
use brickrust::ue::uclass::*;
use brickrust::br::bricks::brick::*;
use brickrust::utils::vtable::*;
use brickworks::{br_print, set_module_name};
set_module_name!("basic init");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust basic init\0".as_ptr(), 
        description: b"This is a mod which makes every single work as RC\n\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.1\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

static mut INITED: bool = false;

/**
 * this function is called during the initialization stage of brickworks
 * 
 * call `brickrust::init` to allow usage of brickrust
 * */
#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    if INITED { return; }
    brickrust::init();
    
    /*
     * because we will be replacing vtable we need to hook on class creation
     * */
    brickrust::hook_construct_uobject(ue_object_init);
    INITED = false;
}

unsafe extern "C" fn is_rc_brick( brick: *mut UBrick ) -> bool
{
    true
}

pub unsafe fn ue_object_init( obj: *mut UObjectBase )
{
    if ((*obj).IsA("UBrick"))
    {
        let brick = obj as *mut UBrick;
        /* 
         * vtable is read only, so we need to replace one with writable ones we malloc 
         * because size is unknown we just estimate it by counting non-null pointers
         * */
        (*brick).uobject.vtable = copy_vtable_estimate_size((*brick).uobject.vtable as *mut usize).0;
        let vtbl = (*brick).uobject.vtable as *mut UBrickVTable;
        (*vtbl).IsRCBrick = is_rc_brick;
    }
}

pub fn frame()
{
}

pub fn deinit()
{
}
