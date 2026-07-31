#![allow(static_mut_refs)]

use brickrust::br::properties::property::{FBrickProperty, FBrickPropertyInstance, FBrickPropertyVTable};
use brickrust::ue::coreuobject::UObject;
use brickrust::ue::tarray::FString;
use brickrust::ue::tpair::TPair;
use brickrust::ue::{fmalloc, fname::*};
use brickrust::ue::tshared::TSharedRef;
use brickrust::{br_print, container_of, modinfo::*};
use brickrust::br::bricks::brick::{UBrick, UBrickVTable};
use brickrust::br::properties::{self, editinfo::*};
use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::br::properties::numeric::*;
use brickrust::utils::vtable::*;
use std::collections::HashMap;
use brickrust::ue::ftext::{BrickRust_string_to_ftext, FText};
use core::mem::zeroed;


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
    pub val: f32,
}

/*
 * we need to use hashmaps, as we cannot simply override object ptr
 * */
static mut BRICKS_HASH: Option<HashMap<*mut UBrick, CustomReceiver>> = None;

#[no_mangle]
pub unsafe extern "C" fn brickrigs_init()
{
    BRICKS_HASH = Some(HashMap::new());
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
    let brick = container_of!(iface, UBrick, property_interface) as *mut UBrick;
    let pvtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    /* 
     * call the parent so we get color etc 
     * position and rotation are processed seperately
     * */
    ((*pvtbl).ReflectBrickProperties)(iface, reflection); 
    
    /*
    if let Some(value) = BRICKS_HASH.as_mut().unwrap().get(&brick)
    {

    }
    */
    let name = FString::from_str("CustomParameter\0");
    let mut text: FName = zeroed();
    BrickRust_string_to_fname(b"CustomParameter\0".as_ptr(), &mut text);
    let mut prop: FNumericBrickPropertyBase = zeroed();
    prop.property.vtable = FNumericBrickPropertyBaseVTable::ptr() as *mut FBrickPropertyVTable;
    prop.property.name = text;
    prop.value_type.value.value = ENumericValueType::Float;
    let pprop = fmalloc::malloc(size_of::<FNumericBrickPropertyBase>(), 0) as *mut FNumericBrickPropertyBase;
    *pprop = prop;
    let mut property = TSharedRef::<FBrickProperty>::make_shared_no_alloc();
    property.object = pprop as *mut FBrickProperty;

    let instance = FBrickPropertyInstance {
        full_name: name,
        property: property,
        parent_chain: zeroed(),
    };
    if (*reflection).is_serializing
    {


        //(*reflection).properties.Add(instance);
    }
    else
    {

        for i in 0..(*reflection).edit_infos.num
        {
            let p = (*reflection).edit_infos.data.add(i as usize);
            let o = (*p).key.object;
            if (*o).instance.full_name.equals_str("BrickSize\0")
            {
                let brickprop = (*o).instance.property.object as *mut FNumericBrickPropertyBase;
                let v = (*brickprop).property.vtable;
                br_print!("{:p} {:#?}", v, (*v));
                (*brickprop).property.vtable = copy_vtable_estimate_size((*brickprop).property.vtable as *mut usize).0 as *mut FBrickPropertyVTable;
                let v = (*brickprop).property.vtable;
                (*v).GetTypeName = FNumericBrickPropertyBase::GetTypeName;
                (*v).GetValueTypeName = FNumericBrickPropertyBase::GetTypeName;
                (*v).SerializeProperty = FBrickPropertyVTable::SerializeProperty;

            }
        }
        /*
        let mut text: FText = zeroed();
        BrickRust_string_to_ftext(b"My parameter".as_ptr(), &mut text);

        let mut edit_info: FBrickPropertyEditInfo = zeroed();
        edit_info.instance = instance;
        edit_info.name = text;
        edit_info.container_objects = (*reflection).container_objects;
        let pedit = TSharedRef::<FBrickPropertyEditInfo>::make_shared();
        *pedit.object = edit_info;
        let p: TPair<TSharedRef<FBrickPropertyEditInfo>, i32> = TPair { key: pedit, v: 0 };
        (*reflection).edit_infos.Add(p);
        */
    }
}

#[no_mangle]
pub unsafe extern "C" fn brickrigs_on_brick_created( brick: *mut UBrick )
{
    let hmap = &mut BRICKS_HASH.as_mut().unwrap();
    hmap.insert(brick, CustomReceiver { val: 5.0 } );

    class_vtable_clone_estimate_size_with_parent(&mut (*brick).property_interface.vtable);
    let vtbl_iface = (*brick).property_interface.vtable as *mut IBrickPropertyInterfaceVTable;
    (*vtbl_iface).ReflectBrickProperties = custom_receiver_reflect_properties;
}
