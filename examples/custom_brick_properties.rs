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
use brickrust::really_scary::uclass_reserve_memory;
use brickrust::really_scary::uclass_reserve_memory2;
use brickrust::ue::fproperty::FProperty;
use brickrust::ue::fproperty::FPropertyVTable;
use brickrust_macros::sig;
use brickworks::br_print;
use brickworks::hookmgr;
use brickworks::modinfo::ModInfo;

use brickrust::br::properties::interface::*;
use brickrust::br::properties::reflection::*;
use brickrust::br::properties::numeric::*;
use brickrust::br::properties::editinfo::*;
use brickrust::br::properties::property::*;
use brickrust::br::bricks::brick::*;

use brickrust::ue::uclass::*;
use brickrust::ue::coreuobject::*;
use brickrust::ue::tarray::*;
use brickrust::ue::fstring::*;
use brickrust::ue::ftext::*;
use brickrust::ue::tshared::*;
use brickrust::ue::fname::*;
use brickrust::ue::tpair::*;

use brickrust::utils::vtable::*;
use brickrust::container_of;

use brickrust::ue::fmalloc;
use brickworks::patterns::*;
use brickworks::set_module_name;

use backtrace;
use std::collections::HashMap;

use core::mem::zeroed;
use std::mem::offset_of;
use std::mem::transmute;
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


static mut IS_IN_OBJECT: Option<unsafe extern "C" fn(a: *mut (), b: *mut ()) -> bool> = None;
unsafe extern "C" fn IsInObject(a: *mut (), b: *mut ()) -> bool
{
    true
}


#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;
    brickrust::init();
    brickrust::hook_construct_uobject(ue_object_init);
    brickrust::hook_post_engine_init(ue_engine_init);
    
    let sig = lookup("IsInObject",sig!("48 89 5c 24 10 56 48 83 ec 20 83 79 28 00"));
    br_print!("{:p}", sig);
    IS_IN_OBJECT = Some(transmute(sig));
    hookmgr::hook(IS_IN_OBJECT.unwrap() as _, IsInObject as _);
    FLOAT_PROPERTY_SERIALIZE_ITEM = lookup("FFloatProperty::SerializeItem", sig!("48 8B 02 4D 8B D0 4C 8B 48 08 49 8b 49 08 48 8b 11 48 8d 42 04"));
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

    let cls = (*brick).uobject.class_private;
    br_print!("{}",(*cls).ustruct.ufield.uobject.name_private);
    br_print!("{}",size_of::<FProperty>());
    let offset = *CUSTOMDATA_OFFSETS.as_mut().unwrap().get(&cls).unwrap();
    let brickdata = (brick as *mut u8).add(offset) as *mut CustomData;

    let mut property = TSharedRef::make_shared_no_alloc();
    property.object = &mut (*brickdata).brickproperty.property;

    let instance = FBrickPropertyInstance {
        full_name: FString::from_str("MyParameter\0"),
        property: property,
        parent_chain: zeroed(),
    };
    if (*reflection).is_serializing
    {
        (*reflection).properties.Add(instance);
    }
    else
    {
        (*brickdata).edit_info._a02 = (*(*(*reflection).edit_infos.data).key.object)._a02;
        let o = (*brickdata).edit_info._a02.unwrap() as *mut UObject;
        let mut r = TSharedRef::make_shared_no_alloc();
        r.object = &mut (*brickdata).edit_info;
        let pair = TPair
        {
            key: r,
            v: 0
        };

        (*brickdata).edit_info.instance = instance;
        (*brickdata).edit_info.container_objects = (*reflection).container_objects.clone_arr();

        (*reflection).edit_infos.Add(pair);
    }
    for p in (*reflection).edit_infos.iter()
    {
        let k = p.key;
        let o = k.object;
        br_print!("{:#?}", *o);
    }
}

#[repr(C)]
struct CustomData
{
    val: f32,
    property: FProperty,
    brickproperty: FNumericBrickPropertyBase,
    edit_info: FBrickPropertyEditInfo,
}
static mut CUSTOMDATA_OFFSETS: Option<HashMap<*const UClass, usize>> = None;
static mut FLOAT_PROPERTY_SERIALIZE_ITEM: *const u8 = core::ptr::null();
unsafe fn ue_engine_init()
{
    CUSTOMDATA_OFFSETS = Some(HashMap::new());
    for o in GObjects().iter()
    {
        br_print!("_ {}", (*o).name_private);
        if (*o).IsA_str("Class")
        {
            let s = o as *mut UStruct;
            if (*s).InheritsFrom_str("Brick")
            {
                let cls = o as *mut UClass;
                CUSTOMDATA_OFFSETS.as_mut().unwrap().insert(cls, uclass_reserve_memory2::<CustomData>(cls));
            }
        }
    }
}
unsafe extern "C" fn SameType( prop: *mut FProperty, other: *mut FProperty) -> bool
{
    (*prop).field.class_private == (*other).field.class_private
}

unsafe fn ue_object_init( _params: FStaticConstructObjectParameters, obj: *mut UObjectBase )
{
    //br_print!("{}", (*obj).name_private);

    if (*obj).IsA_str("Brick")
    {
        let brick = obj as *mut UBrick;
        let cls = (*brick).uobject.class_private;
        br_print!("{}",(*cls).ustruct.ufield.uobject.name_private);
        let offset = *CUSTOMDATA_OFFSETS.as_mut().unwrap().get(&cls).unwrap();
        let data = (brick as *mut u8).add(offset) as *mut CustomData;
        (*data).val = 10.0;

        /*
         * we do not rely on direct usage as we will get memory problems due to reallocations
         * */
        *data = zeroed();

        br_print!("Brick: {:p}, Data {:p}, Offset: {}", brick, data, data as i64 - brick as i64);
        (*data).property.vtbl = fmalloc::calloc2::<FPropertyVTable>(1);
        (*(*data).property.vtbl).SerializeItem = transmute(FLOAT_PROPERTY_SERIALIZE_ITEM);
        (*(*data).property.vtbl).SameType = SameType;
        (*data).property.element_size = 4;
        (*data).property.array_dim = 1;
        (*data).property.offset_internal = offset as u32;

        let n: FName = NAME_NONE;
        let mut prop: FNumericBrickPropertyBase = zeroed();
        prop.property.vtable = FNumericBrickPropertyBase_ptr as *mut FBrickPropertyVTable;
        prop.property.name = n;
        prop.property.property = &mut (*data).property;
        prop.value_type.value.value = ENumericValueType::Float;
        (*data).brickproperty = prop;

        let mut name = FString::from_str("My property\0");
        (*data).edit_info.name = FText::from_fstring(&name);
        name.free();

        let mut name = FString::from_str("My property pretty description banana\0");
        (*data).edit_info.description = FText::from_fstring(&name);
        name.free();

        (*data).edit_info.is_enabled = true;
        //(*data).edit_info.list_items = 1;

        (*brick).property_interface.vtable = copy_vtable_estimate_size_with_parent((*brick).property_interface.vtable as *mut _).0 as *mut _;
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
