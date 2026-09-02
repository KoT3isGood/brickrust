
#![allow(non_snake_case)]

pub mod brickeditor;
pub mod bricks;
pub mod game;
pub mod projectiles;
pub mod properties;
pub mod utils;
pub mod statics;
pub mod modhook;
pub mod vehicle;
pub mod items;
pub mod assetmgr;

use core::mem::transmute;
use brickrust_macros::sig;
use brickworks::patterns::*;

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup("FNumericBrickPropertyBase vtable",sig!("48 8d 05 ?? ?? ?? ?? 66 c7 43 78 00 01"));
    let offset = *(sig.add(3) as *mut u32);
    let sig = sig.add(offset as usize).add(7);
    properties::numeric::FNumericBrickPropertyBase_ptr = transmute(sig);

    game::instance::init_signatures();
    statics::init_signatures();
}
