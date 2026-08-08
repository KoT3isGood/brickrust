#![allow(nonstandard_style)]
pub static mut gworld: *mut *mut UWorld = core::ptr::null_mut();

use crate::ue::coreuobject::EObjectFlags;
use crate::ue::fname::{FName, NAME_NONE};
use crate::ue::uclass::UClass;
use crate::ue::fmath::*;

use super::actor::AActor;

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
pub(crate) static mut SpawnActor_ptr: Option<fnSpawnActor> = None;

impl UWorld
{
    #[inline]
    pub unsafe fn SpawnActor(&mut self, class: *mut UClass, location: *const FVector, rotation: *const FRotator, params: *const FActorSpawnParameters ) -> *mut AActor {
        (SpawnActor_ptr.unwrap())(self, class, location, rotation, params)
    }
}

pub unsafe fn GWorld() -> &'static mut UWorld
{
    return &mut **gworld;
}
