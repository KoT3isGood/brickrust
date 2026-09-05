use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Binary;
use std::mem::MaybeUninit;
use std::mem::zeroed;

use crate::br_print;
use crate::win32::*;
#[cfg(feature = "brmk")]
use crate::brmk::*;
use crate::BrickRust_print;
pub use inventory;
pub use brickrust_macros::sig;
pub use crate::lookup;

#[derive(Debug, Clone, Copy)]
pub struct Signature {
    pub bytes: &'static [u8],
    pub mask: &'static [bool],
}

#[derive(Debug)]
pub struct ExternalFunction
{
    pub signature: Signature,
    pub ptr_fn: *mut usize
}

/**
 * Searches array of bytes for specific signature
 * */
pub unsafe fn lookup_data( data: *const u8, data_len: usize, sign: CSignature ) -> *const u8
{
    let data_len = data_len - sign.num;
    for i in 0..=data_len
    {
        let mut found = true;
        for j in 0..sign.num
        {
            if *sign.mask.add(j) && (*data.add(i+j) != *sign.bytes.add(j))
            {
                found = false;
                break;
            }

        }
        if found {
            return data.add(i);
        }

    }

    return core::ptr::null();
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LookupMode
{
    /// provided as is
    SignatureStart,
    /// for relative addressing
    Offset32,
    /// for direct 64-bit addresses
    Direct64,
}

pub unsafe fn lookup_offset( addr: *const u8, offset: isize, mode: LookupMode) -> *const u8
{
    let addr = addr.offset(offset);
    match mode
    {
        LookupMode::SignatureStart => {
            return addr;
        }
        LookupMode::Offset32 => {
            let reladdr = (addr as *mut u32).read_unaligned();
            return addr.add(4).add(reladdr as usize);
        }
        LookupMode::Direct64 => {
            let reladdr = (addr as *mut u64).read_unaligned();
            return reladdr as *const u8;
        }
    }
}
pub enum LookupInfo
{
    Binary(isize, LookupMode, Signature),
    Proc(&'static str),
}

#[repr(C)]
#[derive(Debug)]
pub struct CSignature {
    pub num: usize,
    pub bytes: *const u8,
    pub mask: *const bool,
}
unsafe extern "C"
{
    pub fn brickworks_binary_lookup( offset: isize, mode: LookupMode, sign: CSignature ) -> *const u8;
    pub fn brickworks_cpp_lookup( cpp: *const u8 ) -> *const u8;
}

pub struct InventoryLookupInfo
{
    pub ptr: *mut *const u8,
    pub info: LookupInfo,
    pub name: &'static str,
}
unsafe impl Sync for InventoryLookupInfo {}

inventory::collect!(InventoryLookupInfo);

#[repr(C)]
pub union LookupValue<T: Copy>
{
    pub generic: *const u8,
    pub typed: T,
}

impl<T: Copy> LookupValue<T>
{
    pub const fn null() -> LookupValue<T> { return LookupValue { generic: core::ptr::null() } }
    pub unsafe fn unwrap(&self) -> T { self.typed }
    pub unsafe fn as_mut_ref(&mut self) -> &mut T { &mut self.typed }
}

#[macro_export]
macro_rules! lookup {
    (
        $(
            pub const $name:ident : $ty:ty = $init:expr;
        )*
    ) => {
        $(
            use $crate::patterns::*;
            pub static mut $name: LookupValue<$ty> = LookupValue::<$ty>::null();
            $crate::patterns::inventory::submit! {
                use $crate::patterns::InventoryLookupInfo;
                InventoryLookupInfo
                {
                    ptr: unsafe { &mut $name.generic },
                    info: $init,
                    name: stringify!($name)
                }
            }
        )*
    };
}

pub unsafe fn do_lookup()
{
    for look in inventory::iter::<InventoryLookupInfo>
    {
        match &look.info
        {
            LookupInfo::Binary(offset, mode, sig) =>
            {
                let sign = CSignature {
                    num: sig.bytes.len(),
                    bytes: sig.bytes.as_ptr(),
                    mask: sig.mask.as_ptr(),
                };
                br_print!("{:#?} {:#?}",sig, mode);
                *look.ptr = brickworks_binary_lookup(*offset, *mode, sign);
                br_print!("{}, {:#?}", look.name,(*look.ptr));
            }
            LookupInfo::Proc(s) =>
            {
                let st = CString::new(*s).unwrap();
                *look.ptr = brickworks_cpp_lookup(st.as_ptr() as *const u8);
            }
        }
    }
}
