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

use core::ffi::c_void;
use core::ffi::CStr;

use brickworks::br_print;
use brickworks::set_module_name;
use brickworks::hookmgr;

use core::mem::transmute;
use ue::fname::*;
use ue::coreuobject::*;
use ue::tarray::FString;
use ue::uclass::*;
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
    ue::init_signatures();
    br::init_signatures();
}

unsafe extern "C" fn static_construct( params: FStaticConstructObjectParameters) -> *mut UObjectBase
{
    let uobject = (StaticConstructObject_Internal_hook.unwrap())(params);
    let (subhooks, count) = hookmgr::get_subhooks(
        transmute(StaticConstructObject_Internal.unwrap())
    );
    let subhooks: *const unsafe fn( obj: *mut UObjectBase ) = transmute(subhooks);
    for i in 0..count
    {
        (*subhooks.add(i))(uobject);
    }
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

static mut INITED: bool = false;

use std::backtrace::Backtrace;
use std::panic;

/**
 * BUG: This is ran twice. Why?
 * */
#[no_mangle]
pub unsafe fn init()
{
    panic::set_hook(Box::new(|info| {
        let bt = Backtrace::force_capture();
        br_print!("Panic: {}", info);
        br_print!("Backtrace:\n{}", bt);
    }));

    if (INITED) { return; }
    init_signatures();

    br_print!("{:#?}", StaticConstructObject_Internal_hook);
    StaticConstructObject_Internal_hook = Some(
        transmute(
            hookmgr::hook(
                StaticConstructObject_Internal.unwrap() as *const (), 
                static_construct as *const ()
            )
        )
    );
    /*
     * todo: make it brickworks thing
     * */
    br_print!("{:#?}", StaticConstructObject_Internal_hook);
    br_print!("initialized brickrust");
    INITED = true;
}

pub unsafe fn hook_construct_uobject( f: unsafe fn( obj: *mut UObjectBase ) )
{
    hookmgr::add_subhook(StaticConstructObject_Internal.unwrap() as *const (), f as *const ());
}
