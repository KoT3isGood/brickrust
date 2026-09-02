#[allow(nonstandard_style)]
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TMulticastDelegate
{
    pub InvocationList: TArray<()>,
    pub CompactionThreshold: i32,
    pub InvocationListLockCount: i32,
}
