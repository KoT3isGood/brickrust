use brickrust::ue::uclass::UObject;
use brickrust::{br_print, container_of, modinfo::*};
use brickrust::br::bricks::brick::{UBrick, UBrickVTable};
use brickrust::br::properties::editinfo::*;
use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::utils::vtable::*;
use std::collections::HashMap;
use brickrust::ue::ftext::FText;


#[no_mangle]
pub unsafe extern "C" fn brickrigs_mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust custom parameters".as_ptr(), 
        description: b"This example shows how is it possible to add brick properties to the editor".as_ptr(), 
        version: b"1.0.0.0".as_ptr(),
        authors: b"BrickRust".as_ptr() 
    }
}

#[repr(C)]
struct CustomReceiver
{
    pub val: u32,
}
#[allow(non_upper_case_globals)]
static mut bricks_hashmap: Option<HashMap<*mut UBrick, CustomReceiver>> = None;

#[no_mangle]
pub unsafe extern "C" fn brickrigs_init()
{
    bricks_hashmap = Some(HashMap::new());
}
#[no_mangle]
pub unsafe extern "C" fn brickrigs_frame()
{
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_deinit()
{
}


pub unsafe extern "C" fn custom_receiver_reflect_properties( iface: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection )
{
    let brick = container_of!(iface, UBrick, property_interface);
    let pvtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    ((*pvtbl).ReflectBrickProperties)(iface, reflection); 
    
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_on_brick_created( mut brick: *mut UBrick )
{
    let hmap = &mut bricks_hashmap.as_mut().unwrap();
    hmap.insert(brick, CustomReceiver { val: 0 } );
    class_vtable_clone_estimate_size_with_parent(&mut (*brick).property_interface.vtable);
    let vtbl_iface = (*brick).property_interface.vtable as *mut IBrickPropertyInterfaceVTable;
    (*vtbl_iface).ReflectBrickProperties = custom_receiver_reflect_properties;
}
