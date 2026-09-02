#![allow(nonstandard_style)]

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FGCObject
{
    pub bReferenceAdded: bool,
}
