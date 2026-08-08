//! This example shows the usage of static functions across the game

#![allow(static_mut_refs)]
use brickrust::ue::fmath::*;
use brickrust::ue::fname::FName;
use brickrust::ue::fstring::FString;
use brickrust::ue::gameplay::world::FActorSpawnParameters;
use brickrust::ue::gameplay::world::GWorld;
use brickworks::set_module_name;
use brickworks::br_print;
use brickrust::bp_function;
use brickworks::modinfo::ModInfo;
use brickrust::ue::tarray::TArray;
use brickrust::ue::coreuobject::*;
use brickrust::ue::uclass::*;
use brickrust::br;
use brickrust;

use microui::*;
use wchar::wch;

set_module_name!(b"function_tests\0");

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"Static function usage\0".as_ptr(), 
        description: b"\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.2\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

unsafe fn test_stuff()
{
    brickrust::warn_version_mismatch!();

    let game_version = br::statics::GetProjectVersion();
    br_print!("Game version: {}", game_version);

    let mut mods = TArray::new();
    br::game::instance::GetEnabledModNames(&mut mods);
    for i in 0..mods.num
    {
        br_print!(": {}", *mods.data.add(i as usize));
    }
}

static mut UCLASS: *mut UClass = core::ptr::null_mut();

unsafe fn engine_init()
{
    for i in 0..GObjects().array.Count()
    {
        let obj = GObjects().array.Get(i);
        let obj = (*obj).object;
        let s = FString::from_fname((*obj).name_private);
        br_print!("{}", s);
        let name = FName::search_str("Class");
        if (*obj).name_private.comparison_index == name.comparison_index
        {
            UCLASS = obj as *mut UClass;
        }
    }
    test_stuff();
}

static mut MICROUI: *mut UClass = core::ptr::null_mut();
unsafe fn engine_loadmap()
{
    let obj = StaticLoadObject(UCLASS, core::ptr::null_mut(), wch!("/microui/microui_render.microui_render_C\0").as_ptr(), core::ptr::null(), 0, false) as *mut UClass;
    let loc = FVector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let rot = FRotator {
        pitch: 0.0,
        yaw: 0.0,
        roll: 0.0,
    };
    let params = FActorSpawnParameters::new();
    let act = GWorld().SpawnActor(obj, &loc, &rot, &params);
}
static mut MICROUI_CTX: mu_Context = unsafe { core::mem::zeroed() };

bp_function!("microui_render_C", microui_input_mouse |obj, stack, result|
{
    #[repr(C)]
    #[derive(Debug)]
    struct Input
    {
        pub x: f32,
        pub y: f32,
    }
    let input = (*stack).locals as *mut Input;
}
);

bp_function!("microui_render_C", microui_begin |obj, stack, result|
{
}
);

bp_function!("microui_render_C", microui_end |obj, stack, result|
{
}
);

bp_function!("microui_render_C", microui_tick |obj, stack, result|
{
});

bp_function!("microui_render_C", microui_drawcall |obj, stack, result|
{
});


#[no_mangle]
pub unsafe extern "C" fn mod_init()
{
    static mut INITED: bool = false;
    if INITED { return; }
    INITED = true;

    microui::backend::init();
    brickrust::init();
    brickrust::hook_post_engine_init(engine_init);
    brickrust::hook_post_load_map(engine_loadmap);
}
