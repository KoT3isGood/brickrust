#![allow(nonstandard_style)]
pub use super::fproperty::*;
pub use super::tarray::*;
pub use super::ffield::*;
pub use super::coreuobject::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UField {
    pub uobject: UObject,
    pub next: *const UField,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FStructBaseChain {
    pub struct_base_chain_array: *const *const FStructBaseChain,
    pub num_struct_bases_in_chain_minus_one: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UStruct {
    pub ufield: UField,
    pub fstruct_base_chain: FStructBaseChain,
    pub super_struct: *const UStruct,
    pub children: *const UField,
    pub child_properties: *const FField,
    pub properties_size: i32,
    pub min_alignment: i32,
    pub script: TArray<u8>,
    pub property_link: *const FProperty,
    pub ref_link: *const FProperty,
    pub destructor_link: *const FProperty,
    pub post_construct_link: *const FProperty,
    pub script_and_property_object_references: TArray<*const UObject>,
    pub unresolved_script_properties: *const (), //TODO pub TArray<TTuple<TFieldPath<FField>,int>,TSizedDefaultAllocator<32> >*
    pub unversioned_schema: *const (),           //TODO const FUnversionedStructSchema*
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UClass {
    pub ustruct: UStruct,
}

/* todo: fix performance issues */
impl UStruct
{
    pub unsafe fn IsChildOf(&self, other: *const UStruct) -> bool
    {
        let mut current = self as *const UStruct;
        loop {
            if current == other
            {
                return true;
            }

            current = (*current).super_struct;

            if current.is_null()
            {
                return false
            }
        }
    }
}

unsafe extern "C"
{
    pub fn BrickRust_GetUStructFromName( child: *const u8, parent: *const u8) -> *const UStruct;
}

#[macro_export]
macro_rules! uclass_game {
    ($a:ty, $module:expr) => {
        use $crate::ue::uclass::{UStruct, StaticClass, BrickRust_GetUStructFromName};
        impl $a {
            pub unsafe fn IsA<T: StaticClass>(&self) -> bool
            {
                let p = ((self) as *const $a) as *const UObject;
                (*p).IsA::<T>()
            }
        }
        impl StaticClass for $a {
            unsafe fn StaticClass() -> *const UStruct
            {
                static mut CLS: *const UStruct = core::ptr::null_mut() as *mut UStruct;
                if CLS.is_null()
                {
                    CLS = BrickRust_GetUStructFromName(
                        concat!(stringify!(#a), "\0").as_ptr(),
                        concat!(stringify!(#b), "\0").as_ptr()
                    )
                }
                return CLS; 

            }
        }
    };
}
