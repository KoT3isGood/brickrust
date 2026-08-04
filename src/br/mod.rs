
#![allow(non_snake_case)]

pub mod brickeditor;
pub mod bricks;
pub mod properties;
pub mod game;
pub mod utils;
pub mod statics;

use core::mem::transmute;
use brickrust_macros::sig;
use brickworks::patterns::*;

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup(sig!("41 FF D0 48 8D 4D E0 48 3B C8 74 ??")).sub(0x3A);
    game::instance::GetEnabledModNames = Some(transmute(sig));

    let sig = lookup(sig!("48 8d 05 ?? ?? ?? ?? 66 c7 43 78 00 01"));
    let offset = *(sig.add(3) as *mut u32);
    let sig = sig.add(offset as usize).add(7);
    properties::numeric::FNumericBrickPropertyBase_ptr = transmute(sig);


    let sig = lookup(sig!("48 83 ec 58 48 85 c9 0f 84 ba 00 00 00"));
    statics::IsModdedAsset_ptr = Some(transmute(sig));
}
