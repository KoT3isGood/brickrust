
use crate::ue::fmalloc;

#[derive(Debug)]
#[repr(C)]
pub struct FReferenceControllerBase {
    pub shared_reference_count: i32,
    pub weak_reference_count: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct TSharedPtr<T> {
    pub object: *mut T,
    pub reference_controller: *mut FReferenceControllerBase,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TWeakPtr<T> {
    pub object: *mut T,
    pub reference_controller: *mut FReferenceControllerBase,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TSharedRef<T> {
    pub object: *mut T,
    pub reference_controller: *mut FReferenceControllerBase,
}
impl<T> TSharedRef<T>
{
    pub fn new() -> TSharedRef<T>
    {
        TSharedRef {
            object: core::ptr::null_mut(),
            reference_controller: core::ptr::null_mut(),
        }
    }
    pub unsafe fn make_shared() -> TSharedRef<T>
    {
        TSharedRef {
            object: fmalloc::malloc(size_of::<T>(), 0) as *mut T,
            reference_controller: fmalloc::malloc(size_of::<FReferenceControllerBase>(), 0) as *mut FReferenceControllerBase,
        }
    }

    pub unsafe fn make_shared_no_alloc() -> TSharedRef<T>
    {
        TSharedRef {
            object: core::ptr::null_mut(),
            reference_controller: fmalloc::malloc(size_of::<FReferenceControllerBase>(), 0) as *mut FReferenceControllerBase,
        }
    }
}
