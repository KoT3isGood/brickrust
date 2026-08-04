//! This example introduces basic 
//!
//! # Limititations
//! - How to add data to the class
//! - How to create FProperties ( they are required by the game )
//! 
//!
//! # What the game does
//! Let's review pseudocode for UScalableBrick.
//!
//! ```cpp
//! virtual void UScalableBrick::ReflectBrickProperties(FBrickPropertyReflection& Params)
//! {
//!     UBrick::ReflectBrickProperties(Params)
//!     BRICK_PROPERTY_CATEGORY(/*CDO.ObjectTypeDisplayName*/)
//!     REFLECT_BRICK_PROPERTY()
//!
//! }
//! ```
//!
//!
//!


#![allow(static_mut_refs)]
use brickrust::ue::fproperty::FProperty;
use brickworks::modinfo::ModInfo;
use brickworks::br_print;

use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::br::properties::numeric::*;
use brickrust::br::properties::editinfo::*;
use brickrust::br::properties::property::*;
use brickrust::br::bricks::brick::*;

use brickrust::ue::uclass::*;
use brickrust::ue::coreuobject::*;
use brickrust::ue::tarray::*;
use brickrust::ue::fname::*;
use brickrust::ue::tpair::*;

use brickrust::utils::vtable::*;
use brickrust::container_of;

use brickrust::ue::fmalloc;
use brickworks::set_module_name;

use std::panic;
use std::backtrace::Backtrace;

use std::collections::HashMap;

use core::mem::zeroed;
set_module_name!(b"custom_brick_properties\0");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust custom properties mod\0".as_ptr(), 
        description: b"Adds custom property to each brick\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.2\0".as_ptr(),
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
    
    BRICKS_HASH = Some(HashMap::new());
    INITED = true;
}

pub unsafe extern "C" fn custom_reflect_properties( iface: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection )
{
    let brick = container_of!(iface, UBrick, property_interface) as *mut UBrick;
    let pvtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    /* 
     * call the parent so we get color etc 
     * position and rotation are processed seperately
     * */
    ((*pvtbl).ReflectBrickProperties)(iface, reflection); 

    let mut brickdata: *mut CustomData = *BRICKS_HASH.as_mut().unwrap().get_mut(&brick).unwrap();

    let name = FString::from_str("CustomParameter\0");
    let mut text: FName = NAME_NONE;
    let mut prop: FNumericBrickPropertyBase = zeroed();
    prop.property.vtable = FNumericBrickPropertyBase_ptr as *mut FBrickPropertyVTable;
    prop.property.name = text;
    prop.value_type.value.value = ENumericValueType::Float;

    (*brickdata).brickproperty = prop;

    let pprop = fmalloc::malloc(size_of::<FNumericBrickPropertyBase>()) as *mut FNumericBrickPropertyBase;
    *pprop = prop;
    let mut property = TSharedRef::<FBrickProperty>::make_shared_no_alloc();
    property.object = pprop as *mut FBrickProperty;

    //br_print!("vtable: {:p}", FNumericBrickPropertyBase_ptr);
    let instance = FBrickPropertyInstance {
        full_name: name,
        property: property,
        parent_chain: zeroed(),
    };
    if (*reflection).is_serializing
    {
        (*reflection).properties.Add(instance);

        for i in 0..(*reflection).properties.num
        {
            let p = (*reflection).properties.data.add(i as usize);
            let bp = (*p).property.object;
            let pp = (*bp).property;
        }
    }
    else
    {
        /*
        for i in 0..(*reflection).edit_infos.num
        {
            let p = (*reflection).edit_infos.data.add(i as usize);
            let o = (*p).key.object;
            br_print!("{:#?}", *p);
            br_print!("{:#?}", *o);
        }
        */
    }
}

#[repr(C)]
struct CustomData
{
    val: f32,
    property: FProperty,
    brickproperty: FNumericBrickPropertyBase
}
/**
 * we cannot use 
 * */
static mut BRICKS_HASH: Option<HashMap<*mut UBrick, *mut CustomData>> = None;

pub unsafe fn ue_object_init( obj: *mut UObjectBase )
{
    if ((*obj).IsA("UBrick"))
    {
        let brick = obj as *mut UBrick;

        let hmap = &mut BRICKS_HASH.as_mut().unwrap();

        /*
         * we do not rely on direct usage as we will get memory problems due to reallocations
         * */
        let data = fmalloc::calloc2::<CustomData>(1);
        hmap.insert( brick, data );

        class_vtable_clone_estimate_size_with_parent(&mut (*brick).property_interface.vtable);

        /**
         * reflection happens through IBrickPropertyInterface
         * */
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
