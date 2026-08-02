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

#[no_std]
set_module_name!(b"brickrust");

pub mod br;
pub mod ue;
pub mod utils;
mod brickrust;

use core::ffi::c_void;
use core::ffi::CStr;

use brickworks::br_print;
use brickworks::set_module_name;
use brickworks::patterns::*;
use brickrust_macros::sig;

use min_hook_rs::enable_hook;
use ue::coreuobject::GOBJECTS_PTR;
use ue::coreuobject::StaticConstructObject_Internal;
use ue::coreuobject::StaticConstructObject_t;
use ue::coreuobject::FStaticConstructObjectParameters;
use ue::uworld::GWORLD_PTR;
use std::string::String;
use core::mem::transmute;

use crate::ue::coreuobject::UObjectBase;
use crate::ue::fname::*;
use crate::ue::tarray::FString;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};

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
    /* from https://github.com/RussellJerome/UnrealModLoader/blob/main/UnrealEngineModLoader/UnrealEngineModLoader/GameInfo/GameInfo.cpp */
    let sig = lookup(sig!("8B 46 10 3B 46 3C 75 0F 48 8B D6 48 8D 0D ? ? ? ? E8"));
    GOBJECTS_PTR = sig.add(18).add(*(sig.add(14) as *mut u32) as usize) as *const *mut ();

    let sig = lookup(sig!("0F 2E ? 74 ? 48 8B 1D ? ? ? ? 48 85 DB 74"));
    GWORLD_PTR = sig.add(12).add(*(sig.add(8) as *mut u32) as usize) as *const *mut ();

    /*
     * each StaticConstructObject_Internal tests for flags
     * we can find these flags
     * */
    let sig = lookup(sig!("F7 86 CC 00 00 00 80 00 00 10")).sub(0x51);
    StaticConstructObject_Internal = Some(transmute(sig));

    let sig = lookup(sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
    GNAMES_PTR = sig.add(9).add(*(sig.add(5) as *mut u32) as usize) as *mut FNamePool;
    
}

unsafe extern "C" fn static_construct( params: FStaticConstructObjectParameters) -> *mut UObjectBase
{
    let uobject = (StaticConstructObject_Internal_hook.unwrap())(params);
    let s = CStr::from_ptr((*uobject).name_private.as_cstr() as *const i8);
    br_print!("{:#?}", s);
    return uobject;
}

#[allow(non_upper_case_globals)]
static mut StaticConstructObject_Internal_hook: Option<StaticConstructObject_t> = None;

struct UnrealModContext
{
    pub construct_object: Option<unsafe fn( obj: *mut UObjectBase )>,

}
static mut CTX: UnrealModContext = UnrealModContext {
    construct_object: None,
};

#[no_mangle]
pub unsafe fn init()
{
    init_signatures();

    /*
     * todo: make it brickworks thing
     * */
    min_hook_rs::initialize();
    if let Ok(hook) = min_hook_rs::create_hook(
        StaticConstructObject_Internal.unwrap() as *mut c_void, 
        static_construct as *mut c_void
        )
    {
        StaticConstructObject_Internal_hook = Some(core::mem::transmute(hook));
    }

    let r = enable_hook(StaticConstructObject_Internal.unwrap() as *mut c_void);
    br_print!("{:#?}", r);
    br_print!("initialized brickrust");
}

pub unsafe fn hook_construct_uobject( f: unsafe fn( obj: *mut UObjectBase ) )
{

}
