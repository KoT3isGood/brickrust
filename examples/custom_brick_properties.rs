#![allow(static_mut_refs)]
use brickworks::modinfo::ModInfo;
use brickworks::br_print;

use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::br::properties::numeric::*;
use brickrust::br::bricks::brick::*;

use brickrust::ue::uclass::*;
use brickrust::ue::coreuobject::*;

use brickrust::utils::vtable::*;
use brickrust::container_of;

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


static mut INITED: bool = false;

#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    if INITED { return; }
    brickrust::init();
    brickrust::hook_construct_uobject(ue_object_init);
    INITED = false;
}

pub unsafe extern "C" fn custom_reflect_properties( iface: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection )
{
    let brick = container_of!(iface, UBrick, property_interface) as *mut UBrick;
    let pvtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    ((*pvtbl).ReflectBrickProperties)(iface, reflection); 
}

pub unsafe fn ue_object_init( obj: *mut UObjectBase )
{
    if ((*obj).IsA("UBrick"))
    {
        let brick = obj as *mut UBrick;
        class_vtable_clone_estimate_size_with_parent(&mut (*brick).property_interface.vtable);
        let vtbl_iface = (*brick).property_interface.vtable as *mut IBrickPropertyInterfaceVTable;
        (*vtbl_iface).ReflectBrickProperties = custom_reflect_properties;
    }
}

pub fn frame()
{
}

pub fn deinit()
{
}
