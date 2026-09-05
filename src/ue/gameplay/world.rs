#![allow(nonstandard_style)]

use crate::ue::coreuobject::EObjectFlags;
use crate::ue::fname::{FName, NAME_NONE};
use crate::ue::uclass::UClass;
use crate::ue::fmath::*;

use super::actor::AActor;
use brickworks::patterns::*;

#[repr(C)]
pub struct UWorld
{

}

#[repr(C)]
pub struct FActorSpawnParameters
{
    pub name: FName,
    pub template: *mut AActor,
    pub owner: *mut AActor,
    pub instigator: *mut AActor,
    pub override_level: *mut (),
    pub flags: u8,
    pub object_flags: EObjectFlags,
}

impl FActorSpawnParameters
{
    pub fn new() -> FActorSpawnParameters
    {
        FActorSpawnParameters {
            name: NAME_NONE,
            template: core::ptr::null_mut(),
            owner: core::ptr::null_mut(),
            instigator: core::ptr::null_mut(),
            override_level: core::ptr::null_mut(),
            flags: 0, 
            object_flags: EObjectFlags::RF_Transactional, 
        }

    }
}

type fnSpawnActor = unsafe extern "C" fn(world: *mut UWorld, class: *mut UClass, location: *const FVector, rotation: *const FRotator, params: *const FActorSpawnParameters ) -> *mut AActor;

lookup!
{
    pub const GWORLD: *mut *mut UWorld = 
        LookupInfo::Binary(0x8, LookupMode::Offset32,sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    pub const SpawnActor_ptr: fnSpawnActor =
        LookupInfo::Binary(-0x33, LookupMode::SignatureStart, sig!("0f 29 44 24 40 0f 29 5c 24 60"));
}

impl UWorld
{
    #[inline]
    pub unsafe fn SpawnActor(&mut self, class: *mut UClass, location: *const FVector, rotation: *const FRotator, params: *const FActorSpawnParameters ) -> *mut AActor {
        (SpawnActor_ptr.unwrap())(self, class, location, rotation, params)
    }
}

pub unsafe fn GWorld() -> &'static mut UWorld
{
    return &mut ***GWORLD.as_mut_ref();
}
