use core::mem::zeroed;
use crate::patterns::*;

unsafe extern "C" fn brmk_hook_call()
{

}

unsafe extern "C" fn brickworks_binary_lookup( offset: isize, mode: LookupMode, sign: Signature ) -> *const u8
{
    core::ptr::null()
}

unsafe extern "C" fn brickworks_cpp_lookup( cpp: &'static str ) -> *const u8
{
    core::ptr::null()
}

pub (crate) struct BRMKLookupInfo<const N: usize> {
    pub dll_names: [*const u8; N],
    pub dll_addresses: [*const u8; N],
    pub dll_sizes: [usize; N],
}

pub (crate) const BRMK_DLLS: BRMKLookupInfo<3> = BRMKLookupInfo
{
    dll_names: [
        b"BrickRigsModKitSteam-BrickRigs.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-Core.dll\0".as_ptr(),
        b"BrickRigsModKitSteam-CoreUObject.dll\0".as_ptr(),
    ],
    dll_addresses: unsafe { zeroed() },
    dll_sizes: unsafe { zeroed() },
};
