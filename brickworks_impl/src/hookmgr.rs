use std::collections::HashMap;
use std::vec::Vec;

use brickworks::{br_print, set_module_name};

unsafe extern "C"
{
    fn brickworks_hook_internal( _f: *const (), _new_fn: *const() ) -> *const ();
}

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
#[no_mangle]
pub unsafe extern "C" fn brickworks_create_hook( f: *const (), new_fn: *const() ) -> *const ()
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

#[no_mangle]
pub unsafe extern "C" fn brickworks_add_prehook( f: *const (), sub: *const () )
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.prehooks.push(sub);
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_get_prehooks( f: *const () ) -> *const *const ()
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    h.prehooks.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_get_prehook_count( f: *const () ) -> usize
{   
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    h.prehooks.len()
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_add_posthook( f: *const (), sub: *const () )
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.posthooks.push(sub);
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_get_posthooks( f: *const () ) -> *const *const ()
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    h.posthooks.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn brickworks_get_posthook_count( f: *const () ) -> usize 
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    h.posthooks.len()
}
