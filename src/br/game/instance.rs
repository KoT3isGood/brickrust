#![allow(non_upper_case_globals)]
use crate::ue::gameplay::instance::UGameInstance;
use crate::ue::tarray::*;
use crate::ue::fstring::*;

use core::mem::transmute;
use brickrust_macros::sig;
use brickworks::patterns::*;

pub static mut GetEnabledModNames_ptr: Option<unsafe extern "C" fn ( out_names: *mut TArray<FString> )> = None;
pub static mut GetEnabledModsHash_ptr: Option<unsafe extern "C" fn () -> i32> = None;

/**
 * > Returns a list of enabled mod names
 *
 * You can use this function to check if blueprint mod is present
 *
 *
 *
 *
 * */

#[inline]
pub unsafe fn GetEnabledModNames( out_names: *mut TArray<FString> )
{
    (GetEnabledModNames_ptr.unwrap())(out_names);
}

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup("UBrickGameInstance::GetEnabledModNames", sig!("4c 8b 42 68 48 8d 55 ?? 41 ff d0 48 8d 4d ??")).sub(0x32);
    GetEnabledModNames_ptr = Some(transmute(sig));

    let sig = lookup("UBrickGameInstance::GetEnabledModsHash", sig!("45 33 ff 48 8d 4c 24 20 41 8b f7")).sub(0x18);
    GetEnabledModsHash_ptr = Some(transmute(sig));
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UBrickGameInstance
{
    pub gameinstance: UGameInstance,
}
