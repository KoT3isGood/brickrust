use crate::ue::coreuobject::UObject;

#[allow(non_upper_case_globals)]
pub static mut IsModdedAsset_ptr: Option<unsafe extern "C" fn( asset: *mut UObject ) -> bool> = None;

#[inline]
pub unsafe fn IsModdedAsset( asset: *mut UObject ) -> bool
{
    (IsModdedAsset_ptr.unwrap())(asset)
}
