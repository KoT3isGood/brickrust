//! This module provides scary functions that allow to commit actual war crimes in code, such as
//! making the UClass allocation size larger for putting own variables.
use crate::ue::fname::FName;
use crate::ue::uclass::*;

pub unsafe fn uclass_reserve_memory( cls: *mut UClass, bytes: usize, alignment: usize ) -> usize
{
    let aligned = ((*cls).ustruct.properties_size + 0xf) & 0xFFFFFFF0;
    let new_size = aligned + bytes as u32;
    let new_size = (new_size + 0xf) & 0xFFFFFFF0;
    (*cls).ustruct.properties_size = new_size;
    (*cls).ustruct.min_alignment = 0x10;

    aligned as usize
}
pub unsafe fn uclass_reserve_memory2<T>( cls: *mut UClass ) -> usize
{
    uclass_reserve_memory( cls, size_of::<T>(), align_of::<T>())
}
