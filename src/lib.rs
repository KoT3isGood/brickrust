#![no_std]
set_module_name!(b"brickrust");

pub mod br;
pub mod ue;
pub mod utils;
mod brickrust;

use core::ffi::c_void;

use brickworks::br_print;
use brickworks::set_module_name;
use brickworks::patterns::*;
use brickrust_macros::sig;

use min_hook_rs::enable_hook;
use ue::coreuobject::GOBJECTS_PTR;
use ue::coreuobject::StaticConstructObject_Internal;
use ue::coreuobject::StaticConstructObject_t;
use ue::coreuobject::FStaticConstructObjectParameters;
use ue::uworld::GWORLD_PTR;

use crate::ue::coreuobject::UObjectBase;

unsafe fn init_signatures()
{
    /* from https://github.com/RussellJerome/UnrealModLoader/blob/main/UnrealEngineModLoader/UnrealEngineModLoader/GameInfo/GameInfo.cpp*/
    let sig = lookup(sig!("8B 46 10 3B 46 3C 75 0F 48 8B D6 48 8D 0D ? ? ? ? E8"));
    GOBJECTS_PTR = sig.add(18).add(*(sig.add(14) as *mut u32) as usize) as *const *mut ();
    br_print!("GObjects: {:p}", *GOBJECTS_PTR);

    let sig = lookup(sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    GWORLD_PTR = sig.add(12).add(*(sig.add(8) as *mut u32) as usize) as *const *mut ();
    br_print!("GWorld: {:p}", *GWORLD_PTR);

    let sig = lookup(sig!("48 8B C8 89 7C 24 ?? E8"));
    StaticConstructObject_Internal = sig as usize + 0x7;
    br_print!("StaticConstructObject_Internal: {:p}", sig.add(0x7));
}

unsafe extern "C" fn static_construct( params: FStaticConstructObjectParameters) -> *mut UObjectBase
{
    br_print!("params: {:#?}", params);
    todo!()
    //(StaticConstructObject_Internal_hook.unwrap())(params)
}
#[allow(non_upper_case_globals)]
static mut StaticConstructObject_Internal_hook: Option<StaticConstructObject_t> = None;

#[no_mangle]
pub unsafe fn init()
{
    init_signatures();

    min_hook_rs::initialize();
    if let Ok(hook) = min_hook_rs::create_hook(
        StaticConstructObject_Internal as *mut c_void, 
        static_construct as *mut c_void
        )
    {
        StaticConstructObject_Internal_hook = Some(core::mem::transmute(hook));
    }

    let r = enable_hook(StaticConstructObject_Internal as *mut c_void);
    br_print!("{:#?}", r);
    br_print!("initialized brickrust");
}

#[no_mangle]
pub unsafe fn hook_ue_init( f: unsafe fn())
{

}
