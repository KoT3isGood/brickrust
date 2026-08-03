
#![allow(non_snake_case)]

pub mod brickeditor;
pub mod bricks;
pub mod properties;
pub mod game;

use core::mem::transmute;
use brickrust_macros::sig;
use brickworks::patterns::*;

pub(crate) unsafe fn init_signatures()
{
    let sig = lookup(sig!("41 FF D0 48 8D 4D E0 48 3B C8 74 ??")).sub(0x3A);
    game::instance::GetEnabledModNames = Some(transmute(sig))
}
