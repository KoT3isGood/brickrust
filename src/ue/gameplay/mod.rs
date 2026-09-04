pub mod actor;
pub mod world;
pub mod assetmanager;
pub mod streamablemgr;
pub mod instance;
use brickworks::patterns::*;
use brickrust_macros::sig;
use world::*;
use core::ptr::*;
use core::mem::transmute;

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup("GWorld",sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    gworld = sig.add(12).add(read_unaligned(sig.add(8) as *mut u32) as usize) as *mut _;

    let sig = lookup("UWorld::SpawnActor",sig!("0f 29 44 24 40 0f 29 5c 24 60")).sub(0x33);
    SpawnActor_ptr = Some(transmute(sig));
}
