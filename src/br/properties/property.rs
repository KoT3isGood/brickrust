
use crate::ue::delegate::TDelegate;
use crate::ue::fname::FName;
use crate::ue::fname::NAME_NONE;
use crate::ue::fproperty::FProperty;
use crate::ue::ftext::FText;
use crate::ue::tshared::TSharedRef;
use crate::ue::fstring::FString;
use crate::ue::tarray::TArray;
use crate::ue::coreuobject::UObject;
use crate::ue::toptional::TOptional;
use brickworks::br_print;
use crate::BrickRust_print;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TBrickPropAttribute<T: Clone>
{
    pub value: TOptional<T>,
    pub delegate: TDelegate,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FBrickPropertyContainer
{
    root_object: *mut UObject,
    container_chain: TArray<*mut ()>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(nonstandard_style)]
pub struct FBrickPropertyVTable
{
    pub GetTypeName: unsafe extern "C" fn( prop: *mut FBrickProperty ) -> FName,
    pub GetValueTypeName: unsafe extern "C" fn( prop: *mut FBrickProperty ) -> FName,
    pub IsOfTypeInternal: unsafe extern "C" fn( prop: *const FBrickProperty, type_name: *const FName ) -> bool,
    pub GetTypeHierarchyInternal: unsafe extern "C" fn( prop: *const FBrickProperty, out_hierarchy: *mut TArray<FName> ),
    pub ScalarDeletingDestructor: unsafe extern "C" fn( prop: *const FBrickProperty ),
    pub u0: unsafe extern "C" fn( prop: *const FBrickProperty ),
    pub ComparePropertyValues: unsafe extern "C" fn( prop: *const FBrickProperty, A: *const (), B: *const () ) -> bool,
    pub u1: unsafe extern "C" fn( prop: *const FBrickProperty ),
    pub u2: unsafe extern "C" fn( prop: *const FBrickProperty ),
    pub SerializeProperty: unsafe extern "C" fn
        ( prop: *mut FBrickProperty, archive: *mut (), container: *const (), ref_resolver: *const () ) -> bool,
    pub DoesObjectContainPropertyInternal: unsafe extern "C" fn( prop: *mut FBrickProperty, in_object: *const UObject ) -> bool,
    pub GetValueAsText: unsafe extern "C" fn( prop: *mut FBrickProperty, container: *const (), out_value: *mut FText ) -> bool,
    pub SetValueAsText: unsafe extern "C" fn( prop: *mut FBrickProperty, container: *const (), new_value: *const FText ) -> bool,
    pub IsUserText: unsafe extern "C" fn( prop: *mut FBrickProperty ) -> bool,
    pub ExportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty, container: *const ()) -> FString,
    pub CanExportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty, container: *const () ) -> bool,
    pub ImportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty, buffer: *const u16 ) -> bool,
    pub CanImportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty, buffer: *const u16 ) -> bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FBrickProperty
{
    pub vtable: *const FBrickPropertyVTable,
    pub property: *const FProperty,
    pub name: FName,
}

#[derive(Debug, Clone, Copy)]
pub struct FBrickPropertyInstance
{
    pub property: TSharedRef<FBrickProperty>,
    pub full_name: FString,
    pub parent_chain: TArray<()>,
}
