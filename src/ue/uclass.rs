#![allow(nonstandard_style)]
use brickworks::br_print;
use brickworks::set_module_name;

use crate::ue::fname::FName;

use super::fproperty::*;
use super::tarray::*;
use super::ffield::*;
use super::coreuobject::*;

pub(crate) static mut UClass_FindFunctionByName_ptr: Option<unsafe extern "C" fn (cls: *const UClass, name: FName, inherit: u32) -> *mut UFunction> = None;
set_module_name!("uclass");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UFieldVTable
{
    pub uobject: UObjectVTable,
    pub AddCppProperty: unsafe extern "C" fn( obj: *mut UField ),
    pub Bind: unsafe extern "C" fn( obj: *mut UField ),
}

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
pub struct UStructVTable
{
    pub ufield: UFieldVTable,
    pub GetInheritanceSuper: unsafe extern "C" fn( obj: *mut UStruct ),
    pub Link: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SerializeBin_C__FStructuredArchiveSlot__Ptr_void: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SerializeBin_C__Ref_FArchive__Ptr_void: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SerializeTaggedProperties_C__FStructuredArchiveSlot__Ptr_uint8__Ptr_UStruct__Ptr_uint8__Ptr_C_UObject: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SerializeTaggedProperties_C__Ref_FArchive__Ptr_uint8__Ptr_UStruct__Ptr_uint8__Ptr_C_UObject: unsafe extern "C" fn( obj: *mut UStruct ),
    pub InitializeStruct: unsafe extern "C" fn( obj: *mut UStruct ),
    pub DestroyStruct: unsafe extern "C" fn( obj: *mut UStruct ),
    pub CustomFindProperty: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SerializeExpr: unsafe extern "C" fn( obj: *mut UStruct ),
    pub GetPrefixCPP: unsafe extern "C" fn( obj: *mut UStruct ),
    pub SetSuperStruct: unsafe extern "C" fn( obj: *mut UStruct ),
    pub PropertyNameToDisplayName: unsafe extern "C" fn( obj: *mut UStruct ),
    pub GetAuthoredNameForField_C__Ptr_C_FField: unsafe extern "C" fn( obj: *mut UStruct ),
    pub GetAuthoredNameForField_C__Ptr_C_UStruct: unsafe extern "C" fn( obj: *mut UField ),
    pub IsStructTrashed: unsafe extern "C" fn( obj: *mut UStruct ),
    pub FindPropertyNameFromGuid: unsafe extern "C" fn( obj: *mut UStruct ),
    pub FindPropertyGuidFromName: unsafe extern "C" fn( obj: *mut UStruct ),
    pub ArePropertyGuidsAvailable: unsafe extern "C" fn( obj: *mut UStruct ),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UStruct {
    pub ufield: UField,
    pub fstruct_base_chain: FStructBaseChain,
    pub super_struct: *const UStruct,
    pub children: *const UField,
    pub child_properties: *const FField,
    pub properties_size: u32,
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
impl UStruct
{
    pub unsafe fn InheritsFrom_fname(&self, name: FName) -> bool
    {
        let mut cls = self.super_struct;
        if self.ufield.uobject.name_private.comparison_index == name.comparison_index
        {
            return true;
        }
        loop 
        {
            if cls.is_null() { break; }
            if (*cls).ufield.uobject.name_private.comparison_index == name.comparison_index
            {
                return true;
            }

            cls = (*cls).super_struct;
        }
        false
    }
    pub unsafe fn InheritsFrom_str(&self, name: &'static str) -> bool
    {
        let fname = FName::search_str(name);
        self.InheritsFrom_fname(fname)
    }
    pub unsafe fn dump_inheritance(&self)
    {
        let mut cls = self.super_struct;
        loop 
        {
            if cls.is_null() { break; }
            br_print!("{}", (*cls).ufield.uobject.name_private);
            cls = (*cls).super_struct;
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UClassVTable
{
    pub ustruct: UStructVTable,
    pub GetAuthoritativeClass: unsafe extern "C" fn( obj: *mut UClass ),
    pub PostInitInstance: unsafe extern "C" fn( obj: *mut UClass ),
    pub InitPropertiesFromCustomList: unsafe extern "C" fn( obj: *mut UClass ),
    pub SetupObjectInitializer: unsafe extern "C" fn( obj: *mut UClass ),
    pub GetPersistentUberGraphFrame: unsafe extern "C" fn( obj: *mut UClass ),
    pub CreatePersistentUberGraphFrame: unsafe extern "C" fn( obj: *mut UClass ),
    pub DestroyPersistentUberGraphFrame: unsafe extern "C" fn( obj: *mut UClass ),
    pub SerializeDefaultObject__Ptr_UObject__Ref_FArchive: unsafe extern "C" fn( obj: *mut UClass ),
    pub SerializeDefaultObject__Ptr_UObject__FStructuredArchiveSlot: unsafe extern "C" fn( obj: *mut UClass ),
    pub PostLoadDefaultObject: unsafe extern "C" fn( obj: *mut UClass ),
    pub PurgeClass: unsafe extern "C" fn( obj: *mut UClass ),
    pub IsFunctionImplementedInScript: unsafe extern "C" fn( obj: *mut UClass ),
    pub HasProperty: unsafe extern "C" fn( obj: *mut UClass ),
    pub FindArchetype: unsafe extern "C" fn( obj: *mut UClass ),
    pub GetArchetypeForCDO: unsafe extern "C" fn( obj: *mut UClass ),
    pub GetArchetypeForSparseClassData: unsafe extern "C" fn( obj: *mut UClass ),
    pub GetDefaultObjectPreloadDependencies: unsafe extern "C" fn( obj: *mut UClass ),
    pub CreateDefaultObject: unsafe extern "C" fn( obj: *mut UClass ),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UClass {
    pub ustruct: UStruct,
    pub ClassConstructor: usize,
    pub ClassVTableHelperCtorCaller: usize,
    pub ClassAddReferencedObjects: usize,
    pub flags: u32,
    pub ClassFlags1: u64,
    pub ClassFlags2: u64,
    pub ClassCastFlags:u64,
    pub ClassWithin: *mut UClass,
    pub ClassGeneratedBy: *mut UObject,
    pub ClassConfigName: FName,
    pub NetFields: TArray<*mut UField>,
    pub FirstOwnedClassRep: i32,
    pub ClassDefaultObject: *mut UObject,
    pub SparseClassData: *mut (),
    pub SparseClassDataStruct: *mut (),
}

impl UClass
{
    pub unsafe fn FindFunctionByName(&self, name: &'static str, inherit: bool ) -> *mut UFunction
    {
        let n = FName::search_str(name);
        (UClass_FindFunctionByName_ptr.unwrap())(self, n, inherit as u32)
    }
    pub unsafe fn GetDefaultObject(&mut self, create_if_needed: bool) -> *mut UObject
    {
        if self.ClassDefaultObject.is_null() && create_if_needed
        {
            let v = self.ustruct.ufield.uobject.vtable as *mut UClassVTable;
            ((*v).CreateDefaultObject)(self);
        }
        return self.ClassDefaultObject;
    }

}

/*
 * Todo
 * */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UFunction {
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
    //pub fn BrickRust_GetUStructFromName( child: *const u8, parent: *const u8) -> *const UStruct;
}

/*
#[macro_export]
macro_rules! uclass_game {
    ($a:ty, $module:expr) => {
        use $crate::ue::uclass::{UStruct, BrickRust_GetUStructFromName};
        use $crate::ue::coreuobject::{StaticClass};
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
*/
