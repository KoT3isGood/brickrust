#![allow(nonstandard_style)]

use crate::ue::{coreuobject::GObjects, fmalloc};

#[derive(Debug, Clone, Copy)]
pub struct FReferenceControllerBaseVTable {
    pub DestroyObject: unsafe extern "C" fn ( controller: *mut FReferenceControllerBase ),
    pub _destructor: unsafe extern "C" fn ( controller: *mut FReferenceControllerBase ),
}

impl FReferenceControllerBaseVTable
{
    unsafe extern "C" fn DestroyObject( _controller: *mut FReferenceControllerBase )
    {

    }
    unsafe extern "C" fn _destructor( _controller: *mut FReferenceControllerBase )
    {

    }
    pub fn new() -> *const FReferenceControllerBaseVTable
    {
        const VTABLE: FReferenceControllerBaseVTable = FReferenceControllerBaseVTable
        {
            DestroyObject: FReferenceControllerBaseVTable::DestroyObject,
            _destructor: FReferenceControllerBaseVTable::_destructor,
        };
        return &VTABLE;
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FReferenceControllerBase {
    pub vtable: *const FReferenceControllerBaseVTable,
    pub shared_reference_count: i32,
    pub weak_reference_count: i32,
}
impl FReferenceControllerBase
{
    pub fn new() -> FReferenceControllerBase
    {
        FReferenceControllerBase { 
            vtable: FReferenceControllerBaseVTable::new(),
            shared_reference_count: 1,
            weak_reference_count: 0
        }

    }

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
        let refcon = fmalloc::calloc2::<FReferenceControllerBase>(1);
        *refcon = FReferenceControllerBase::new();
        TSharedRef {
            object: fmalloc::malloc(size_of::<T>()) as *mut T,
            reference_controller: refcon,
        }
    }

    pub unsafe fn make_shared_no_alloc() -> TSharedRef<T>
    {
        let refcon = fmalloc::calloc2::<FReferenceControllerBase>(1);
        *refcon = FReferenceControllerBase::new();
        TSharedRef {
            object: core::ptr::null_mut(),
            reference_controller: refcon,
        }
    }
    pub unsafe fn unwrap(&self) -> *mut T
    {
        return self.object;
    }
}
