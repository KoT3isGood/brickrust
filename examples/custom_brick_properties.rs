//! This example introduces basic brick properties for the game.
//! It shows how to create numeric brick property which allows to enable and disable RC Brick
//! properties
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
use std::mem::offset_of;
use std::mem::zeroed;

use brickrust::br::properties::editinfo::FBrickPropertyEditInfo;
use brickrust::br::properties::numeric::ENumericValueType;
use brickrust::br::properties::numeric::FNumericBrickPropertyBase;
use brickrust::br::properties::numeric::FNumericBrickPropertyRange;
use brickrust::br::properties::numeric::FNumericBrickPropertyValue;
use brickrust::br::properties::numeric::NUMERIC_BRICK_PROPERTY_FLOAT_VTABLE;
use brickrust::br::properties::property::FBrickProperty;
use brickrust::br::properties::property::FBrickPropertyInstance;
use brickrust::br::properties::property::TBrickPropAttribute;
use brickrust::ue::fmalloc::calloc_from_object;
use brickrust::ue::fname::FName;
use brickrust::ue::fproperty::FProperty;
use brickrust::ue::fstring::FString;
use brickrust::ue::ftext::FText;
use brickrust::ue::tarray::TArray;
use brickrust::ue::toptional::TOptional;
use brickrust::ue::tpair::TPair;
use brickrust::ue::tshared::TSharedRef;
use brickrust::ue::tshared::TWeakPtr;
use brickworks::br_print;
use brickworks::modinfo::ModInfo;

use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::br::bricks::brick::*;

use brickrust::ue::coreuobject::*;


use brickrust::utils::vtable::*;
use brickrust::container_of;

use brickworks::set_module_name;

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

#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    brickrust::init();
    brickrust::hook_construct_uobject(ue_object_init);
}

pub unsafe extern "C" fn custom_reflect_properties( iface: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection )
{
    let brick = container_of!(iface, UBrick, property_interface) as *mut UBrick;
    let vtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    ((*vtbl).ReflectBrickProperties)(iface, reflection);

    let cls = (*brick).uobject.class_private;
    let st = &(*cls).ustruct;
    let prop = st.GetProperty_str("FloatProperty");

    let numericprop = FNumericBrickPropertyBase
    {
        property: FBrickProperty {
            vtable: NUMERIC_BRICK_PROPERTY_FLOAT_VTABLE.unwrap(),
            property: prop,
            name: FName::search_str("FloatProperty\0"),
        },
        value_type: TBrickPropAttribute
        {
            value: TOptional::some(ENumericValueType::Float),
            delegate: zeroed(),
        },
        _a0: 0,
        _a1: 0,
        _a2: 0,
        _a3: 0,
        _a4: 0,
        _a5: 0,
        _a6: 0,
        _a7: 0,
        _a8: 0,
        _a9: 0,
        _a10: 0,
        _a11: 0,
        axis_lock: TBrickPropAttribute
        {
            value: TOptional::some(brickrust::br::properties::numeric::EFluAxisLock::None),
            delegate: zeroed(),
        },
        /*
        value_range: TBrickPropAttribute
        {
            value: TOptional::some(FNumericBrickPropertyRange::from_f32(-100.0, 100.0)),
            delegate: zeroed(),
        },

    */
    };
    let mut numericprop_ref = TSharedRef::make_shared_no_alloc();
    numericprop_ref.object = calloc_from_object(&numericprop) as *mut FBrickProperty;

    let instance = FBrickPropertyInstance {
        property: numericprop_ref,
        full_name: FString::from_str("FloatProperty\0"),
        parent_chain: TArray::new(),
    };
    

    
    //br_print!("{:#?}", *reflection);

    if (*reflection).is_serializing
    {
        (*reflection).properties.Add(instance);
        return;
    }
    if !(*reflection).is_serializing
    {
        let edit_info_ref = TSharedRef::<FBrickPropertyEditInfo>::make_shared();
        let edit_info = edit_info_ref.object;
        *edit_info = zeroed();
        (*edit_info).instance = instance;
        (*edit_info).is_enabled = true;
        (*edit_info).container_objects = (*reflection).container_objects.clone_arr();
        (*edit_info).this = TWeakPtr {
            object: edit_info_ref.object,
            reference_controller: edit_info_ref.reference_controller,
        };
        (*edit_info_ref.reference_controller).weak_reference_count += 1;
        //(*edit_info)._a01 = 0x00000000ffffffff;
        (*edit_info)._a05 = (*reflection)._a0;

        let mut name = FString::from_str("FloatProperty\0");
        (*edit_info).name = FText::from_fstring(&name);
        name.free();
        
        let mut name = FString::from_str("Hello world!\0");
        (*edit_info).description = FText::from_fstring(&name);
        name.free();

        let pair = TPair {
            key: edit_info_ref,
            v: 0,
        };
        (*reflection).edit_infos.Add(pair);

    }
}
/*
pub unsafe extern "C" fn custom_reflect_properties2( iface: *const IBrickPropertyInterface, reflection: *mut FBrickPropertyReflection )
{
    let vtbl = class_get_parent_vtable(iface as *mut _) as *mut IBrickPropertyInterfaceVTable;

    ((*vtbl).ReflectBrickProperties)(iface, reflection);
    for key in (*reflection).edit_infos.iter()
    {
        let edit = *key.key.object;
        if edit.instance.full_name.equals_str("BrickSize\0")
        {
            br_print!("{:#?}", *(edit.instance.property.object as *mut FNumericBrickPropertyBase));
            br_print!("{:x}", offset_of!(FNumericBrickPropertyBase, axis_lock));
            br_print!("{:x}", size_of::<FNumericBrickPropertyBase>());
        }
    }
}
*/

unsafe fn ue_object_init( _params: *mut FStaticConstructObjectParameters, obj: *mut UObjectBase )
{
    if (*obj).IsA_str("MyPropertiesBrick_C")
    {
        let brick = obj as *mut UBrick;
        (*brick).property_interface.vtable = copy_vtable_estimate_size_with_parent((*brick).property_interface.vtable as *mut _).0 as *mut _;
        let vtbl_iface = (*brick).property_interface.vtable as *mut IBrickPropertyInterfaceVTable;
        (*vtbl_iface).ReflectBrickProperties = custom_reflect_properties;
    }
    /*
    if (*obj).IsA_str("ScalableBrick")
    {
        let brick = obj as *mut UBrick;
        (*brick).property_interface.vtable = copy_vtable_estimate_size_with_parent((*brick).property_interface.vtable as *mut _).0 as *mut _;
        let vtbl_iface = (*brick).property_interface.vtable as *mut IBrickPropertyInterfaceVTable;
        (*vtbl_iface).ReflectBrickProperties = custom_reflect_properties2;
    }
    */
}

pub fn frame()
{
}

pub fn deinit()
{
}
