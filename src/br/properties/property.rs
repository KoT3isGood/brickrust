
use crate::ue::fname::FName;
use crate::ue::fproperty::FProperty;
use crate::ue::tshared::TSharedRef;
use crate::ue::tarray::FString;
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FBrickPropertyVTable
{
    pub GetTypeName: unsafe extern "C" fn( prop: *mut FBrickProperty ) -> FName,
    pub GetValueTypeName: unsafe extern "C" fn( prop: *mut FBrickProperty ) -> FName,
    pub IsOfTypeInternal: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub GetTypeHierarchyInternal: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub GetTypeHierarchy: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub ComparePropertyValues: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub SerializeProperty: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub DoesObjectContainPropertyInternal: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub GetValueAsText: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub SetValueAsText: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub IsUserText: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub ExportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub CanExportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub ImportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty ),
    pub CanImportProperty: unsafe extern "C" fn( prop: *mut FBrickProperty ),
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBrickProperty
{
    pub vtable: *mut FBrickPropertyVTable,
    pub property: *mut FProperty,
    pub name: FName,
}

#[derive(Debug, Clone)]
pub struct FBrickPropertyInstance
{
    pub property: TSharedRef<FBrickProperty>,
    pub full_name: FString,
    pub parent_chain: TArray<()>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FTextBrickProperty
{
    pub prop: FBrickProperty,
    pub max_text_lenght: i32,
    pub password: bool,
    pub multiline: bool,
    pub user_text: bool
}
