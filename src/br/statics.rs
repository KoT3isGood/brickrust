#![allow(non_upper_case_globals)]

use core::mem::transmute;
use brickrust_macros::sig;
use brickworks::{patterns::*, set_module_name};

use crate::ue::coreuobject::UObject;
use crate::ue::fstring::*;


pub static mut GetProjectVersion_ptr: Option<unsafe extern "C" fn() -> FString> = None;
pub static mut IsModdedAsset_ptr: Option<unsafe extern "C" fn( asset: *mut UObject ) -> bool> = None;
set_module_name!(b"statics\0");

/**
 * > Returns the project version string
 * Returns game version, for example `1.11.2`
 */
#[inline]
pub unsafe fn GetProjectVersion() -> FString {
    (GetProjectVersion_ptr.unwrap())()
}

/**
 * > Returns true if the asset is part of a plugin/mod
 * */
#[inline]
pub unsafe fn IsModdedAsset( asset: *mut UObject ) -> bool {
    (IsModdedAsset_ptr.unwrap())(asset)
}

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup2("UBrickStatics::GetProjectVersion", 0x2B, LookupMode::SignatureStart,sig!("48 8b 93 18 01 00 00 48 8b cf 48 81 c2 b8 00 00 00"));
    GetProjectVersion_ptr = Some(transmute(sig));
    
    let sig = lookup2("UBrickStatics::IsModdedAsset", 0, LookupMode::SignatureStart, sig!("48 83 ec 58 48 85 c9 0f 84 ?? ?? ?? ??"));
    IsModdedAsset_ptr = Some(transmute(sig));
}
