pub mod actor;
pub mod world;
pub mod assetmanager;
pub mod streamablemgr;
pub mod instance;
use brickworks::patterns::*;
use brickrust_macros::sig;
use world::*;
use core::mem::transmute;

pub(crate) unsafe fn init_signatures()
{
    gworld = lookup2("GWorld", 0x8, LookupMode::Offset32,sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74")) as *mut _;
    let sig = lookup2("UWorld::SpawnActor", -0x33, LookupMode::SignatureStart, sig!("0f 29 44 24 40 0f 29 5c 24 60"));
    SpawnActor_ptr = Some(transmute(sig));
}
