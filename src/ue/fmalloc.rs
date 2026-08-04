
#![allow(non_upper_case_globals)]

pub const DEFAULT_ALIGNMENT: u32 = 0u32;
use brickrust_macros::lookup;

pub(crate) static mut Malloc: 
Option<unsafe extern "C" fn ( count: usize, align: u32 ) -> *mut ()> = None;

pub(crate) static mut Realloc: 
Option<unsafe extern "C" fn ( original: *mut(), count: usize, align: u32 ) -> *mut ()> = None;

pub(crate) static mut Free: 
Option<unsafe extern "C" fn ( original: *mut() ) -> bool> = None;


pub unsafe fn malloc( count: usize ) -> *mut ()
{
    (Malloc.unwrap())(count, DEFAULT_ALIGNMENT)
}

pub unsafe fn calloc2<T>( count: usize ) -> *mut T
{
    (Malloc.unwrap())( count * size_of::<T>(), DEFAULT_ALIGNMENT) as *mut T
}

pub unsafe fn realloc( original: *mut(), count: usize ) -> *mut ()
{
    (Realloc.unwrap())(original, count, DEFAULT_ALIGNMENT)
}

pub unsafe fn free( original: *mut() ) -> bool
{
    (Free.unwrap())(original)
}
