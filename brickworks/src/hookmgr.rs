use std::collections::HashMap;
use std::vec::Vec;
use core::ffi::c_void;

use crate::win32::brickworks_hook_internal;


#[derive(Clone)]
struct Hook
{
    f: *const (),
    prehooks: Vec<*const ()>,
    posthooks: Vec<*const ()>,
}

static mut HOOKS: Option<HashMap<*const (), Hook>> = None;


pub(crate) unsafe fn init()
{
    HOOKS = Some(HashMap::new());
    let _ = min_hook_rs::initialize();

}
/**
 * Creates and enables hooking for a function.
 * Returns hooked pointer to the function.
 * */
pub unsafe fn hook( f: *const (), new_fn: *const() ) -> *const ()
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_some() { 
        return r.unwrap().f;
    }

    let mut h: Hook = Hook { 
        f: core::ptr::null(), 
        prehooks: Vec::new(),
        posthooks: Vec::new(),
    };

    h.f = brickworks_hook_internal(f, new_fn);

    hooks.insert(f, h.clone());
    h.f
}
pub unsafe fn add_prehook( f: *const (), sub: *const () )
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.prehooks.push(sub);
}
pub unsafe fn add_posthook( f: *const (), sub: *const () )
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.posthooks.push(sub);
}

pub unsafe fn get_prehooks( f: *const () ) -> (*const *const (), usize)
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    return (h.prehooks.as_ptr(), h.prehooks.len());
}

pub unsafe fn get_posthooks( f: *const () ) -> (*const *const (), usize)
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    return (h.posthooks.as_ptr(), h.posthooks.len());
}
