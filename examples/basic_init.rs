use brickrust::{br_print, modinfo::*};
use brickrust::br::bricks::brick::{UBrick, UBrickVTable};
use brickrust::utils::vtable::*;

#[no_mangle]
pub unsafe extern "C" fn brickrigs_mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust basic init".as_ptr(), 
        description: b"This is a mod which makes every single work as RC".as_ptr(), 
        version: b"1.0.0.0".as_ptr(),
        authors: b"BrickRust".as_ptr() 
    }
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_init()
{
}
#[no_mangle]
pub unsafe extern "C" fn brickrigs_frame()
{
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_deinit()
{
}

unsafe extern "C" fn basic_is_rc( _brick: *mut UBrick ) -> bool
{
    return true;
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_on_brick_created( brick: *mut UBrick )
{
    class_vtable_clone_estimate_size(&mut (*brick).uobject.vtable);
    let vtbl = (*brick).uobject.vtable as *mut UBrickVTable;
    (*vtbl).IsRCBrick = basic_is_rc;
}
