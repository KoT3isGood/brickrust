use std::collections::HashMap;
use std::vec::Vec;
use min_hook_rs::*;
use core::ffi::c_void;


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
        subhooks: Vec::new(),
    };

    let r = create_hook( f as *mut c_void, new_fn as *mut c_void );
    if r.is_err() { return core::ptr::null(); }
    h.f = core::mem::transmute(r.unwrap());

    let r = enable_hook(f as *mut c_void);
    if r.is_err() { return core::ptr::null(); }

    hooks.insert(f, h.clone());
    h.f
}

pub unsafe fn add_subhook( f: *const (), sub: *const () )
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get_mut(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &mut Hook = r.unwrap();
    h.subhooks.push(sub);

}
pub unsafe fn get_subhooks( f: *const () ) -> (*const *const (), usize)
{
    let hooks = HOOKS.as_mut().unwrap();
    let r = hooks.get(&f);
    if r.is_none() { panic!("function not found: {:p}", f) }
    let h: &Hook = r.unwrap();
    return (h.subhooks.as_ptr(), h.subhooks.len());
}
