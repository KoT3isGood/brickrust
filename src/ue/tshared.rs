
use crate::ue::{coreuobject::GObjects, fmalloc};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FReferenceControllerBase {
    pub shared_reference_count: i32,
    pub weak_reference_count: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TSharedPtr<T> {
    pub object: *mut T,
    pub reference_controller: *mut FReferenceControllerBase,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TWeakPtr<T> {
    pub object: *mut T,
    pub reference_controller: *mut FReferenceControllerBase,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FWeakObjectPtr {
    pub object_index: i32,
    pub object_serial_number: i32,
}

impl FWeakObjectPtr
{
    pub unsafe fn unwrap<T>(&self) -> *mut T
    {
        (*GObjects().array.Get(self.object_index)).object as *mut T
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
            object: fmalloc::malloc(size_of::<T>()) as *mut T,
            reference_controller: fmalloc::malloc(size_of::<FReferenceControllerBase>()) as *mut FReferenceControllerBase,
        }
    }

    pub unsafe fn make_shared_no_alloc() -> TSharedRef<T>
    {
        TSharedRef {
            object: core::ptr::null_mut(),
            reference_controller: fmalloc::malloc(size_of::<FReferenceControllerBase>()) as *mut FReferenceControllerBase,
        }
    }
    pub unsafe fn unwrap(&self) -> *mut T
    {
        return self.object;
    }
}
