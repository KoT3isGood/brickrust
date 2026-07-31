use brickworks::modinfo::ModInfo;
use brickworks::br_print;

#[no_mangle]
extern "C" fn mod_info() -> ModInfo
{
    ModInfo { 
        name: b"BrickRust basic init\0".as_ptr(), 
        description: b"This is a mod which makes every single work as RC\0".as_ptr(), 
        version: b"1.0.0.0\0".as_ptr(),
        game_version: b"1.11.1\0".as_ptr(),
        authors: b"BrickRust\0".as_ptr() 
    }
}

pub fn init()
{
}

pub fn frame()
{
}

pub fn deinit()
{
}
