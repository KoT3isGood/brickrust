
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
pub struct TBrickPropAttribute<T>
{
    pub value: TOptional<T>,
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

impl FBrickPropertyVTable
{
    pub const fn new() -> FBrickPropertyVTable
    {
        FBrickPropertyVTable { 
            GetTypeName: FBrickPropertyVTable::GetTypeName,
            GetValueTypeName: FBrickPropertyVTable::GetValueTypeName,
            IsOfTypeInternal: FBrickPropertyVTable::IsOfTypeInternal,
            GetTypeHierarchyInternal: FBrickPropertyVTable::GetTypeHierarchyInternal,
            u0: FBrickPropertyVTable::unknown,
            ScalarDeletingDestructor: FBrickPropertyVTable::ScalarDeletingDestructor,
            u1: FBrickPropertyVTable::unknown,
            u2: FBrickPropertyVTable::unknown,
            ComparePropertyValues: FBrickPropertyVTable::ComparePropertyValues,
            SerializeProperty: FBrickPropertyVTable::SerializeProperty,
            DoesObjectContainPropertyInternal: FBrickPropertyVTable::DoesObjectContainPropertyInternal,
            GetValueAsText: FBrickPropertyVTable::GetValueAsText,
            SetValueAsText: FBrickPropertyVTable::SetValueAsText,
            IsUserText: FBrickPropertyVTable::IsUserText,
            ExportProperty: FBrickPropertyVTable::ExportProperty,
            CanExportProperty: FBrickPropertyVTable::CanExportProperty,
            ImportProperty: FBrickPropertyVTable::ImportProperty,
            CanImportProperty: FBrickPropertyVTable::CanImportProperty,
        }
    }
    pub unsafe extern "C" fn unknown( _prop: *const FBrickProperty )
    {
        br_print!("called unknown");
    }
    pub unsafe extern "C" fn GetTypeName( _prop: *mut FBrickProperty ) -> FName
    {
        br_print!("GetTypeName");
        NAME_NONE
    }
    pub unsafe extern "C" fn GetValueTypeName( _prop: *mut FBrickProperty ) -> FName
    {
        br_print!("GetValueTypeName");
        NAME_NONE
    }
    pub unsafe extern "C" fn IsOfTypeInternal( _prop: *const FBrickProperty, _type_name: *const FName ) -> bool
    {
        br_print!("IsOfTypeInternal");
        false
    }
    pub unsafe extern "C" fn GetTypeHierarchyInternal( _prop: *const FBrickProperty, _out_hierarchy: *mut TArray<FName> )
    {
        br_print!("GetTypeHierarchyInternal");
        todo!()
    }
    pub unsafe extern "C" fn GetTypeHierarchy( _prop: *const FBrickProperty, _out_hierarchy: *mut TArray<FName> )
    {
        br_print!("GetTypeHierarchy");
        ((*(*_prop).vtable).GetTypeHierarchyInternal)(_prop, _out_hierarchy)
    }
    pub unsafe extern "C" fn ScalarDeletingDestructor( _prop: *const FBrickProperty )
    {
        br_print!("ScalarDeletingDestructor");
    }
   
    pub unsafe extern "C" fn ComparePropertyValues( _prop: *const FBrickProperty, _A: *const (), _B: *const () ) -> bool
    {
        br_print!("ComparePropertyValues");
        panic!("User must implement this function and shouldn't call it")
    }
    pub unsafe extern "C" fn SerializeProperty
        ( _prop: *mut FBrickProperty, archive: *mut (), container: *const (), ref_resolver: *const () ) -> bool
    {
        br_print!("SerializeProperty {:p} {:p} {:p}", archive, container, ref_resolver);
        true
    }
    pub unsafe extern "C" fn DoesObjectContainPropertyInternal( _prop: *mut FBrickProperty, _in_object: *const UObject ) -> bool
    {
        br_print!("DoesObjectContainPropertyInternal");
        true
    }
    pub unsafe extern "C" fn GetValueAsText( _prop: *mut FBrickProperty, _container: *const (), _out_value: *mut FText ) -> bool
    {
        br_print!("GetValueAsText");
        false
    }

    pub unsafe extern "C" fn SetValueAsText( _prop: *mut FBrickProperty, _container: *const (), _new_value: *const FText ) -> bool
    {
        br_print!("SetValueAsText");
        false
    }
    pub unsafe extern "C" fn IsUserText( _prop: *mut FBrickProperty ) -> bool
    {
        br_print!("IsUserText");
        false
    }

    pub unsafe extern "C" fn ExportProperty( _prop: *mut FBrickProperty, _container: *const () ) -> FString
    {
        br_print!("ExportProperty");
        FString::new()
    }
    pub unsafe extern "C" fn CanExportProperty( _prop: *mut FBrickProperty, _container: *const () ) -> bool
    {
        br_print!("CanExportProperty");
        false
    }
    pub unsafe extern "C" fn ImportProperty( _prop: *mut FBrickProperty, _buffer: *const u16 ) -> bool
    {
        br_print!("ImportProperty");
        false
    }
    pub unsafe extern "C" fn CanImportProperty( _prop: *mut FBrickProperty, _buffer: *const u16 ) -> bool
    {
        br_print!("CanImportProperty");
        false
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FBrickProperty
{
    pub vtable: *mut FBrickPropertyVTable,
    pub property: *mut FProperty,
    pub name: FName,
}

#[derive(Debug, Clone, Copy)]
pub struct FBrickPropertyInstance
{
    pub property: TSharedRef<FBrickProperty>,
    pub full_name: FString,
    pub parent_chain: TArray<()>,
}
