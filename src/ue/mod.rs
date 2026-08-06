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
pub mod uworld;
pub mod utils;
pub mod blueprint;

use brickworks::br_print;
use brickworks::patterns::*;
use brickworks::hookmgr;
use brickrust_macros::sig;
use brickworks::set_module_name;
use coreuobject::*;
use uworld::*;
use crate::ue::fname::*;
use crate::ue::tarray::FString;
use core::mem::transmute;
use fframe::FFrame;
set_module_name!(b"ue\0");

pub(crate) static mut UEngine_Init_ptr: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;
static mut UEngine_Init_hook: Option<unsafe extern "C" fn (a: *mut (), b: *mut ())> = None;

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

unsafe extern "C" fn process_internal(obj: *mut UObject, stack: *mut FFrame, result: *mut ())
{
    let obj_name = (*obj).name_private;
    let func_name = (*(*stack).node).ustruct.ufield.uobject.name_private;
    #[cfg(feature = "process_internal_debug")]
    {
        let obj_str = FString::from_fname(obj_name);
        let func_str = FString::from_fname(func_name);
        br_print!("{} {}", obj_str, func_str);
    }

    use super::blueprint::BlueprintFunction;
    /*
     * Todo: we can do better using hashmaps
     * */
    for f in inventory::iter::<BlueprintFunction>
    {
        let fn_fname = FName::search_str(f.function_name);

        let fn_str = FString::from_fname(fn_fname);
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

pub(crate) unsafe fn init_signatures()
{
    /* from https://github.com/RussellJerome/UnrealModLoader/blob/main/UnrealEngineModLoader/UnrealEngineModLoader/GameInfo/GameInfo.cpp */
    let sig = lookup("GObjects",sig!("8B 46 10 3B 46 3C 75 0F 48 8B D6 48 8D 0D ? ? ? ? E8"));
    GOBJECTS_PTR = sig.add(18).add(*(sig.add(14) as *mut u32) as usize) as *mut ();

    let sig = lookup("GWorld",sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    GWORLD_PTR = sig.add(12).add(*(sig.add(8) as *mut u32) as usize) as *const *mut ();

    let sig = lookup("GNames",sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
    GNAMES_PTR = sig.add(9).add(*(sig.add(5) as *mut u32) as usize) as *mut FNamePool;

    let sig = lookup("FMemory::Realloc",sig!("48 8b fa 48 85 c9 75 0c")).sub(0x1C);
    fmalloc::Realloc = Some(transmute(sig));
    
    let sig = lookup("FMemory::Malloc",sig!("48 8b f9 8b da 48 8b 0d ?? ?? ?? ?? 48 85 c9")).sub(0xA);
    fmalloc::Malloc = Some(transmute(sig));

    let sig = lookup("FMemory::Free",sig!("48 85 c9 74 2e 53"));
    fmalloc::Free = Some(transmute(sig));

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

    let sig = lookup("UObject::ProcessInternal", sig!("48 8b d9 ff 90 28 02 00 00")).sub(0x2B);
    ProcessInternal_ptr = Some(transmute(sig));
    ProcessInternal_hook = Some(
        transmute(
            hookmgr::hook(
                ProcessInternal_ptr.unwrap() as *const _, 
                process_internal as *const _
            )
        )
    );
}
