#![allow(non_upper_case_globals)]

use brickworks::{br_print, patterns::*};

pub const DEFAULT_ALIGNMENT: u32 = 0u32;

lookup! {
    pub const Malloc: unsafe extern "C" fn ( count: usize, align: u32 ) -> *mut () = 
        LookupInfo::Binary(-0xA, LookupMode::SignatureStart, sig!("48 8b f9 8b da 48 8b 0d ?? ?? ?? ?? 48 85 c9"));
    pub const Realloc: unsafe extern "C" fn ( original: *mut(), count: usize, align: u32 ) -> *mut () =
        LookupInfo::Binary(-0x1C, LookupMode::SignatureStart,sig!("48 8b fa 48 85 c9 75 0c"));
    pub const Free: unsafe extern "C" fn ( original: *mut() ) -> bool =
        LookupInfo::Binary(0x0, LookupMode::SignatureStart,sig!("48 85 c9 74 2e 53"));
}
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
