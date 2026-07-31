#![no_std]

use brickworks::br_print;
use brickworks::set_module_name;

pub mod br;
pub mod ue;
pub mod utils;
mod brickrust;

set_module_name!(b"brickrust");

#[no_mangle]
unsafe extern "C" fn brickrust_init()
{
    br_print!("initializing brickrust");
}
