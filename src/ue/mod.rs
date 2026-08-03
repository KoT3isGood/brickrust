pub mod coreuobject;
pub mod ffield;
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

use brickworks::patterns::*;
use brickrust_macros::sig;
use coreuobject::*;
use uworld::GWORLD_PTR;
use crate::ue::fname::*;
use core::mem::transmute;

pub(crate) unsafe fn init_signatures()
{
    /* from https://github.com/RussellJerome/UnrealModLoader/blob/main/UnrealEngineModLoader/UnrealEngineModLoader/GameInfo/GameInfo.cpp */
    let sig = lookup(sig!("8B 46 10 3B 46 3C 75 0F 48 8B D6 48 8D 0D ? ? ? ? E8"));
    GOBJECTS_PTR = sig.add(18).add(*(sig.add(14) as *mut u32) as usize) as *mut ();

    let sig = lookup(sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    GWORLD_PTR = sig.add(12).add(*(sig.add(8) as *mut u32) as usize) as *const *mut ();

    let sig = lookup(sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
    GNAMES_PTR = sig.add(9).add(*(sig.add(5) as *mut u32) as usize) as *mut FNamePool;

    /*
     * each StaticConstructObject_Internal tests for flags
     * we can find these flags
     * */
    let sig = lookup(sig!("F7 86 CC 00 00 00 80 00 00 10")).sub(0x51);
    StaticConstructObject_Internal = Some(transmute(sig));
}
