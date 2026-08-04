use std::collections::HashMap;
use std::vec::Vec;
use std::mem::zeroed;
use min_hook_rs::*;
use core::ffi::c_void;

use crate::BrickRust_print;
use crate::br_print;

struct Subhook
{

}

#[derive(Clone)]
struct Hook
{
    f: *const (),
    subhooks: Vec<*const ()>,
}

static mut HOOKS: Option<HashMap<*const (), Hook>> = None;


pub(crate) unsafe fn init()
{
    HOOKS = Some(HashMap::new());
    min_hook_rs::initialize();

}

pub unsafe fn hook( f: *const (), new_fn: *const() ) -> *const ()
{
    let mut hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_some() { 
        return r.unwrap().f;
    }

    let mut h: Hook = Hook { 
        f: core::ptr::null(), 
        subhooks: Vec::new(),
    };

    let r = create_hook( f as *mut c_void, new_fn as *mut c_void );
    if r.is_err() { return core::ptr::null(); }
    h.f = core::mem::transmute(r.unwrap());

    let r = enable_hook(f as *mut c_void);
    if r.is_err() { return core::ptr::null(); }

    br_print!("{:p}", f);
    hooks.insert(f, h.clone());
    h.f
}

pub unsafe fn add_subhook( f: *const (), sub: *const() )
{
    let mut hooks = HOOKS.as_mut().unwrap();
    let mut r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.subhooks.push(sub);

}
pub unsafe fn get_subhooks( f: *const () ) -> (*const *const (), usize)
{
    let mut hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    return (h.subhooks.as_ptr(), h.subhooks.len());
}
