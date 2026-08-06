//! This crate is a library for using Brick Rigs
//!
//! It provides headers for the engine and the game.
//!
//! 
//! 
//! # Creating simple mods
//! 
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

set_module_name!(b"brickrust\0");

pub mod br;
pub mod ue;
pub mod utils;
mod brickrust;


use brickworks::br_print;
use brickworks::set_module_name;
use brickworks::hookmgr;

use ue::coreuobject::*;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
use ue::UEngine_Init_ptr;

pub(crate) unsafe fn disassemble( data: *const u8 )
{
    let bytes = core::slice::from_raw_parts(data, 200);
    let mut decoder =
        Decoder::with_ip(64, bytes, data as u64, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::new();
    formatter.options_mut().set_digit_separator("`");
    formatter.options_mut().set_first_operand_char_index(10);
    
    let mut output = String::new();

    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);

        output.clear();
        formatter.format(&instruction, &mut output);
        br_print!("{}", output);
    }
}

unsafe fn init_signatures()
{
    ue::init_signatures();
    br::init_signatures();
}

use std::backtrace::Backtrace;
use std::panic;

use crate::ue::blueprint;
use crate::ue::tarray::TArray;

/**
 * BUG: This is ran twice. Why?
 * */
#[no_mangle]
pub unsafe fn init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;

    panic::set_hook(Box::new(|info| {
        let bt = Backtrace::force_capture();
        br_print!("Panic: {}", info);
        br_print!("Backtrace:\n{}", bt);
    }));

    init_signatures();
    blueprint::init();
}

pub unsafe fn hook_construct_uobject( f: unsafe fn( obj: *mut UObjectBase ) )
{
    hookmgr::add_subhook(StaticConstructObject_Internal.unwrap() as *const (), f as *const ());

}
pub unsafe fn hook_post_engine_init( f: unsafe fn() )
{
    hookmgr::add_subhook(UEngine_Init_ptr.unwrap() as *const (), f as *const ());
}

#[macro_export]
macro_rules! warn_version_mismatch {
    () => {
        let mi = mod_info();
        let ver = $crate::br::statics::GetProjectVersion();
        if ver.equals_cstr(mi.game_version) == false
        {
            br_print!("Version mismatch!")
        }
    };
}

#[macro_export]
macro_rules! panic_version_mismatch {
    () => {
        let mi = mod_info();
        let ver = $crate::br::statics::GetProjectVersion();
        if ver.equals_cstr(mi.game_version) == false
        {
            panic!("Version mismatch!")
        }
    };
}

pub unsafe fn check_blueprint_mod( mod_name: &'static str) -> bool
{
    let mut arr = TArray::new();
    br::game::instance::GetEnabledModNames(&mut arr);
    for i in 0..arr.num
    {
        let m = arr.data.add(i as usize);
        if (*m).equals_str( mod_name )
        {
            return true;
        }
    }
    false
}

pub unsafe fn ensure_blueprint_mod( m: &'static str)
{
    if check_blueprint_mod(m) == false
    {
        panic!("Mod must be present for app to work: {}", m)
    }
}
