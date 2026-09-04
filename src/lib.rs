//! This crate is a library for using Brick Rigs and Unreal Engine.
//! It provides tooling to create mods with [brickworks]
//! 
//! # Creating simple mods
//!
//! ## Your `Cargo.toml`
//! Your mod must be packed as dylib for it to work properly with [brickworks].
//!
//! ```toml
//! [package]
//! name = "your_mod"
//! edition = "2021" # preferred for crust-like experience
//!
//! [lib]
//! crate-type = ["dylib"]
//! ```
//! 
//! Now you can compile your mod using `cargo build --target x86_64-pc-windows-gnu`
//! ~~(it is the only one we support for now)~~
//!
//!
//! ## Initialization 
//! All mods using [brickworks] must have `mod_info` and `mod_init` functions.
//! You can copy over this template.
//!
//! ```
//! #[no_mangle]
//! unsafe extern "C" fn mod_info() -> ModInfo
//! {
//!     ModInfo { 
//!         name: b"Mod name\0".as_ptr(), 
//!         description: b"Mod description\0".as_ptr(), 
//!         version: b"0.0\0".as_ptr(),
//!         game_version: b"1.11.2\0".as_ptr(), // used for runtime checks
//!         authors: b"you\0".as_ptr() 
//!     }
//! }
//!
//! #[no_mangle]
//! unsafe extern "C" fn mod_init()
//! {
//!     brickworks::init()
//!     /* your code*/
//! }
//! ```
//!
//! `mod_init` is ran before the engine is initialized, so it is not recommended to run engine
//! functions for spawning actors, etc.
//!
//! ## Running engine functions
//! To access different engine modules different engine initialization is required.
//! To provide this pipeline the hooks are provides
//!
//! You can run these function during mod_init to hook the engine using
//! [`hook_post_engine_init`] and map using [`hook_post_load_map`]
//! ```
//! /* required to access static game functions and other stuff */
//! brickrust::hook_post_engine_init(engine_init);
//! /* required to GWorld() */
//! brickrust::hook_post_load_map(engine_loadmap);
//! ```
//!
//! You can also hook static construct of UObjects using [`hook_construct_uobject`] to modify
//! UObjects at their creations. For example to replace vTables [`utils::vtable`]
//!
//! ```
//! brickrust::hook_construct_uobject(engine_construct_uobject)
//! ```
//!
//! ## Runtime dependency checks
//! Your mods can validation multiple conditions for mod to run.
//! This has been added to ensure vTable and structure compatibility.
//! All of them must be ran after engine initialization.
//! - To validate game version you can use [`warn_version_mismatch`] and [`panic_version_mismatch`].
//! - To validate presence of blueprint mod you can use [`check_blueprint_mod`] and
//! [`ensure_blueprint_mod`].
//! 
//! # Installing a mod
//!
//! Copy compiled mod library from target directory to the `steamapps/Brick Rigs/brickworks`.
//! For any library dependencies they must be put in `steamapps/Brick Rigs`.
//!
set_module_name!(b"brickrust\0");

pub mod br;
pub mod ue;
pub mod utils;
pub mod really_scary;
mod brickrust;


use brickworks::br_print;
use brickworks::set_module_name;
use brickworks::hookmgr;

use ue::coreuobject::*;
use ue::*;

unsafe fn init_signatures()
{
    ue::init_signatures();
    br::init_signatures();
}

use crate::ue::blueprint;
use crate::ue::tarray::TArray;
use crate::ue::uclass::UClass;

pub unsafe fn autobacktrace()
{    
    let bt = backtrace::Backtrace::new();
    for frame in bt.frames() {
        let ip = frame.ip();
        br_print!("{:?}:", ip);

        for symbol in frame.symbols() {
            if let Some(name) = symbol.name() {
                br_print!("  {}", name);
            }
        }
    }
}

/**
 * Sets up all the signatures for engine interactions.
 * You cannot call any engine and game functions before calling this.
 * */
#[no_mangle]
pub unsafe fn init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;


    init_signatures();
    blueprint::init();
}

pub unsafe fn hook_construct_uobject( f: unsafe fn( params: FStaticConstructObjectParameters, obj: *mut UObjectBase ) )
{
    hookmgr::add_subhook(StaticConstructObject_Internal.unwrap() as *const (), f as *const ());
}
pub unsafe fn hook_load_uobject( f: unsafe fn( obj: *mut UObjectBase, class: *mut UClass, in_outer: *mut UObject, inname: *const u16, filename: *const u16, flags: u32 ) )
{
    hookmgr::add_subhook(StaticLoadObject_ptr.unwrap() as *const (), f as *const ());
}

pub unsafe fn hook_post_engine_init( f: unsafe fn() )
{
    hookmgr::add_subhook(UEngine_Init_ptr.unwrap() as *const (), f as *const ());
}

pub unsafe fn hook_post_load_map( f: unsafe fn() )
{
    hookmgr::add_subhook(UEngine_LoadMap_ptr.unwrap() as *const (), f as *const ());
}

/**
 * Drops a warning when the version doesn't match the one provided in the mod's description.
 *
 * Cannot be ran during mod initialization
 * */
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

/**
 * Panics when the version doesn't match the one provided in the mod's description.
 *
 * Cannot be ran during mod initialization
 * */
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

/** 
 * Returns true if the mod is present
 *
 * Cannot be ran during mod initialization
 * */
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

/** 
 * Panics if the mod is not present
 *
 * Cannot be ran during mod initialization
 * */
pub unsafe fn ensure_blueprint_mod( m: &'static str)
{
    if check_blueprint_mod(m) == false
    {
        panic!("Mod must be present for app to work: {}", m)
    }
}
