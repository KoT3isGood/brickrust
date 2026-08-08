#![allow(non_upper_case_globals)]
pub mod coreuobject;
pub mod ffield;
pub mod fframe;
pub mod fmalloc;
pub mod fmath;
pub mod fname;
pub mod fproperty;
pub mod fstring;
pub mod ftext;
pub mod ftagcontainer;
pub mod tarray;
pub mod toptional;
pub mod tpair;
pub mod farchive;
pub mod tshared;
pub mod uclass;
pub mod utils;
pub mod blueprint;
pub mod gameplay;

use brickworks::br_print;
use brickworks::patterns::*;
use brickworks::hookmgr;
use brickrust_macros::sig;
use brickworks::set_module_name;
use coreuobject::*;
use uclass::*;
use fname::*;
use fstring::FString;
use core::mem::transmute;
use fframe::FFrame;
set_module_name!(b"ue\0");

pub(crate) static mut UEngine_Init_ptr: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;
static mut UEngine_Init_hook: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;

pub(crate) static mut UEngine_LoadMap_ptr: Option<unsafe extern "C" fn (a: *mut (), b: *mut (), c: *mut (), d: *mut (), e: *mut ()) -> bool> = None;
static mut UEngine_LoadMap_hook: Option<unsafe extern "C" fn (a: *mut (), b: *mut (), c: *mut (), d: *mut (), e: *mut ()) -> bool> = None;

static mut StaticConstructObject_Internal_hook: Option<StaticConstructObject_t> = None;

unsafe extern "C" fn static_construct( params: FStaticConstructObjectParameters) -> *mut UObjectBase
{
    let uobject = (StaticConstructObject_Internal_hook.unwrap())(params);
    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(StaticConstructObject_Internal.unwrap())
    );
    let subhooks: *const unsafe fn( obj: *mut UObjectBase ) = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))(uobject);
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
    /*
     * Todo: we can do better using hashmaps
     * */
    for f in inventory::iter::<BlueprintFunction>
    {
        let fn_fname = FName::search_str(f.function_name);

        match f.class
        {
            None => {
            }
            Some(c) =>
            {
                let cls_fname = FName::search_str(c);
                if cls_fname.comparison_index != obj_name.comparison_index
                {
                    continue;
                }
            }
        }
        if fn_fname.comparison_index != func_name.comparison_index
        {
            continue;
        }
        (f.function)(obj, stack, result);
        return;
    }
    (ProcessInternal_hook.unwrap())(obj, stack, result)
}
use core::ptr::read_unaligned;

use crate::hook_post_engine_init;

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

    let sig = lookup("GObjects",sig!("0F AF EA 41 FF C9"));
    GOBJECTS_PTR = sig.add(12).add((read_unaligned(sig.add(8) as *mut u32)) as usize) as *mut _;

    let sig = lookup("GNames",sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
    GNAMES_PTR = sig.add(9).add(read_unaligned(sig.add(5) as *mut u32) as usize) as *mut _;

    let sig = lookup("FMemory::Realloc",sig!("48 8b fa 48 85 c9 75 0c")).sub(0x1C);
    fmalloc::Realloc = Some(transmute(sig));
    
    let sig = lookup("FMemory::Malloc",sig!("48 8b f9 8b da 48 8b 0d ?? ?? ?? ?? 48 85 c9")).sub(0xA);
    fmalloc::Malloc = Some(transmute(sig));

    let sig = lookup("FMemory::Free",sig!("48 85 c9 74 2e 53"));
    fmalloc::Free = Some(transmute(sig));

    let sig = lookup("UObject::StaticLoadObject", sig!("48 33 c4 48 89 85 80 02 00 00 0f b6 85 10 03 00 00")).sub(0x23);
    StaticLoadObject_ptr = Some(transmute(sig));

    let sig = lookup("UClass::FindFunctionByName", sig!(" 8b 81 38 01 00 00 45 8b f0 48 8b da 48 8b e9")).sub(0xD);
    UClass_FindFunctionByName_ptr = Some(transmute(sig));

    let sig = lookup("UEngine::Init",sig!("48 8d 6c 24 d9 48 81 ec 00 01 00 00 4c 8b f1")).sub(0x10);
    UEngine_Init_ptr = Some(transmute(sig));

    UEngine_Init_hook = Some(
        transmute(
            hookmgr::hook(
                UEngine_Init_ptr.unwrap() as *const _, 
                engine_init as *const _
            )
        )
    );

    let sig = lookup("UEngine::LoadMap",sig!("4c 89 74 24 60 4c 8b ea 4c 89 4c 24 30 4c 89 44 24 70 48 89 4c 24 50")).sub(0x3C);
    UEngine_LoadMap_ptr = Some(transmute(sig));

    UEngine_LoadMap_hook = Some(
        transmute(
            hookmgr::hook(
                UEngine_LoadMap_ptr.unwrap() as *const _, 
                engine_loadmap as *const _
            )
        )
    );

    let sig = lookup("UObject::StaticConstructObject_Internal", sig!("F7 86 CC 00 00 00 80 00 00 10")).sub(0x51);
    StaticConstructObject_Internal = Some(transmute(sig));

    StaticConstructObject_Internal_hook = Some(
        transmute(
            hookmgr::hook(
                StaticConstructObject_Internal.unwrap() as *const _, 
                static_construct as *const _
            )
        )
    );

    let sig = lookup("ProcessLocalScriptFunction", sig!("80 f9 04 74 34 66 66 66 0f 1f 84 00 00 00 00 00 48 ff c0")).sub(0x30);
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
