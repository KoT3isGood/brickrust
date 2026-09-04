#![allow(non_upper_case_globals)]
pub mod coreuobject;
pub mod farrayserializer;
pub mod ffield;
pub mod fframe;
pub mod fmalloc;
pub mod fmath;
pub mod fname;
pub mod fproperty;
pub mod fstring;
pub mod ftext;
pub mod ftagcontainer;
pub mod fexec;
pub mod tarray;
pub mod toptional;
pub mod tpair;
pub mod tmap;
pub mod farchive;
pub mod tshared;
pub mod uclass;
pub mod utils;
pub mod blueprint;
pub mod gameplay;
pub mod gcobject;
pub mod delegate;

use brickworks::br_print;
use brickworks::patterns::*;
use brickworks::hookmgr;
use brickrust_macros::sig;
use brickworks::set_module_name;
use coreuobject::*;
use uclass::*;
use fname::*;
use core::mem::transmute;
use fframe::FFrame;
set_module_name!(b"ue\0");

pub(crate) static mut UEngine_Init_ptr: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;
static mut UEngine_Init_hook: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;

pub(crate) static mut UEngine_LoadMap_ptr: Option<unsafe extern "C" fn (a: *mut (), b: *mut (), c: *mut (), d: *mut (), e: *mut ()) -> bool> = None;
static mut UEngine_LoadMap_hook: Option<unsafe extern "C" fn (a: *mut (), b: *mut (), c: *mut (), d: *mut (), e: *mut ()) -> bool> = None;

static mut StaticConstructObject_Internal_hook: Option<StaticConstructObject_t> = None;

unsafe extern "C" fn static_load( class: *mut UClass, in_outer: *mut UObject, inname: *const u16, filename: *const u16, flags: u32, reconciliation: bool ) -> *mut UObjectBase
{
    let uobject = (StaticLoadObject_hook.unwrap())(class, in_outer, inname, filename, flags, reconciliation);
    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(StaticLoadObject_ptr.unwrap())
    );
    let subhooks: *const unsafe fn( obj: *mut UObjectBase, class: *mut UClass, in_outer: *mut UObject, inname: *const u16, filename: *const u16, flags: u32 ) = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))(uobject, class, in_outer, inname, filename, flags);
    }
    return uobject;
}

unsafe extern "C" fn static_construct( params: FStaticConstructObjectParameters ) -> *mut UObjectBase
{
    let uobject = (StaticConstructObject_Internal_hook.unwrap())(params);
    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(StaticConstructObject_Internal.unwrap())
    );
    let subhooks: *const unsafe fn( params: FStaticConstructObjectParameters, obj: *mut UObjectBase ) = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))(params, uobject);
    }
    return uobject;
}
unsafe extern "C" fn engine_init(a: *mut (), b: *mut ())
{
    (UEngine_Init_hook.unwrap())(a, b);

    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(UEngine_Init_ptr.unwrap())
    );

    let subhooks: *const unsafe fn() = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))();
    }
}

unsafe extern "C" fn engine_loadmap(a: *mut (), b: *mut (), c: *mut (), d: *mut (), e: *mut ()) -> bool
{
    let r = (UEngine_LoadMap_hook.unwrap())(a, b, c, d, e);

    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(UEngine_LoadMap_ptr.unwrap())
    );

    let subhooks: *const unsafe fn() = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))();
    }
    r
}

unsafe extern "C"
{

}

unsafe extern "C" fn process_internal(obj: *mut UObject, stack: *mut FFrame, result: *mut ())
{
    let obj_name = (*obj).name_private;
    let func_name = (*(*stack).node).ustruct.ufield.uobject.name_private;

    use super::blueprint::BlueprintFunction;
    for f in inventory::iter::<BlueprintFunction>
    {

        match f.class
        {
            None => {
            }
            Some(c) =>
            {
                if !obj_name.equals_str(c)
                {
                    continue;
                }
            }
        }
        if !func_name.equals_str(f.function_name)
        {
            continue;
        }
        (f.function)(obj, &mut *stack, result);
        return;
    }
    (ProcessInternal_hook.unwrap())(obj, stack, result)
}
use core::ptr::read_unaligned;

use crate::hook_post_engine_init;
use crate::ue::ftext::Conv_StringToText;

unsafe fn engine_load()
{
    for i in 0..GObjects().array.Count()
    {
        let obj = GObjects().array.Get(i);
        let obj = (*obj).object;
        let name = FName::search_str("Class");
        if (*obj).name_private.comparison_index == name.comparison_index
        {
            UCLASS = obj as *mut UClass;
        }
    }
}

pub(crate) unsafe fn init_signatures()
{
    gameplay::init_signatures();

    GOBJECTS_PTR = lookup2("GObjects", 8, LookupMode::Offset32, sig!("0F AF EA 41 FF C9")) as *mut _;
    GNAMES_PTR = lookup2("GNames", 5, LookupMode::Offset32,sig!("74 09 48 8D 15 ? ? ? ? EB 16")) as *mut _;
    
    let sig = lookup2("Conv_StringToText", 24, LookupMode::Offset32, sig!("74 41 48 8d 54 24 20 48 8b c8 e8 ?? ?? ?? ?? 48 8b d0 48 8d 4c 24 30"));
    Conv_StringToText = Some(transmute(sig));

    let sig = lookup2("FMemory::Realloc", -0x1C, LookupMode::SignatureStart,sig!("48 8b fa 48 85 c9 75 0c"));
    fmalloc::Realloc = Some(transmute(sig));
    
    let sig = lookup2("FMemory::Malloc", -0xA, LookupMode::SignatureStart, sig!("48 8b f9 8b da 48 8b 0d ?? ?? ?? ?? 48 85 c9"));
    fmalloc::Malloc = Some(transmute(sig));

    let sig = lookup2("FMemory::Free", 0x0, LookupMode::SignatureStart,sig!("48 85 c9 74 2e 53"));
    fmalloc::Free = Some(transmute(sig));


    let sig = lookup2("UClass::FindFunctionByName", -0xD, LookupMode::SignatureStart, sig!("8b 81 38 01 00 00 45 8b f0 48 8b da 48 8b e9"));
    UClass_FindFunctionByName_ptr = Some(transmute(sig));

    let sig = lookup2("UEngine::Init", -0x10, LookupMode::SignatureStart, sig!("48 8d 6c 24 d9 48 81 ec 00 01 00 00 4c 8b f1"));
    UEngine_Init_ptr = Some(transmute(sig));

    UEngine_Init_hook = Some(
        transmute(
            hookmgr::hook(
                UEngine_Init_ptr.unwrap() as *const _, 
                engine_init as *const _
            )
        )
    );

    let sig = lookup2("UEngine::LoadMap", -0x3C, LookupMode::SignatureStart, sig!("4c 89 74 24 60 4c 8b ea 4c 89 4c 24 30 4c 89 44 24 70 48 89 4c 24 50"));
    UEngine_LoadMap_ptr = Some(transmute(sig));

    UEngine_LoadMap_hook = Some(
        transmute(
            hookmgr::hook(
                UEngine_LoadMap_ptr.unwrap() as *const _, 
                engine_loadmap as *const _
            )
        )
    );

    let sig = lookup2("UObject::StaticConstructObject_Internal", -0x51, LookupMode::SignatureStart, sig!("F7 86 CC 00 00 00 80 00 00 10"));
    br_print!("{:p}", sig);
    StaticConstructObject_Internal = Some(transmute(sig));

    StaticConstructObject_Internal_hook = Some(
        transmute(
            hookmgr::hook(
                StaticConstructObject_Internal.unwrap() as *const _, 
                static_construct as *const _
            )
        )
    );

    let sig = lookup2("UObject::StaticLoadObject", -0x23, LookupMode::SignatureStart, sig!("48 33 c4 48 89 85 80 02 00 00 0f b6 85 10 03 00 00"));
    StaticLoadObject_ptr = Some(transmute(sig));
    StaticLoadObject_hook = Some(
        transmute(
            hookmgr::hook(
                StaticLoadObject_ptr.unwrap() as *const _, 
                static_load as *const _
            )
        )
    );

    let sig = lookup2("ProcessLocalScriptFunction", -0x30, LookupMode::SignatureStart, sig!("80 f9 04 74 34 66 66 66 0f 1f 84 00 00 00 00 00 48 ff c0"));
    ProcessInternal_ptr = Some(transmute(sig));
    ProcessInternal_hook = Some(
        transmute(
            hookmgr::hook(
                ProcessInternal_ptr.unwrap() as *const _, 
                process_internal as *const _
            )
        )
    );

    hook_post_engine_init(engine_load);
}
