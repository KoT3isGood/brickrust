use super::ffield::FField;
use super::fname::FName;

#[repr(C)]
#[derive(Debug)]
#[allow(nonstandard_style)]
pub struct FPropertyVTable
{
    pub __vecDelDtor: unsafe extern "C" fn(),
    pub Serialize: unsafe extern "C" fn(),
    pub PostLoad: unsafe extern "C" fn(),
    pub GetPreloadDependencies: unsafe extern "C" fn(),
    pub BeginDestroy: unsafe extern "C" fn(),
    pub AddReferencedObjects: unsafe extern "C" fn(),
    pub AddCppProperty: unsafe extern "C" fn(),
    pub Bind: unsafe extern "C" fn(),
    pub PostDuplicate: unsafe extern "C" fn(),
    pub GetInnerFieldByName: unsafe extern "C" fn(),
    pub GetInnerFields: unsafe extern "C" fn(),
    pub GetCPPMacroType: unsafe extern "C" fn(),
    pub PassCPPArgsByRef: unsafe extern "C" fn(),
    pub GetCPPType: unsafe extern "C" fn(),
    pub GetCPPTypeForwardDeclaration: unsafe extern "C" fn(),
    pub LinkInternal: unsafe extern "C" fn(),
    pub ConvertFromType: unsafe extern "C" fn(),
    pub Identical: unsafe extern "C" fn(),
    pub SerializeItem: unsafe extern "C" fn(),
    pub NetSerializeItem: unsafe extern "C" fn(),
    pub SupportsNetSharedSerialization: unsafe extern "C" fn(),
    pub ExportTextItem: unsafe extern "C" fn(),
    pub ImportText_Internal: unsafe extern "C" fn(),
    pub CopyValuesInternal: unsafe extern "C" fn(),
    pub GetValueTypeHashInternal: unsafe extern "C" fn(),
    pub CopySingleValueToScriptVM: unsafe extern "C" fn(),
}

#[repr(C)]
#[derive(Debug)]
pub struct FProperty {
    pub vtbl: *mut FPropertyVTable,
    pub field: FField,
    pub array_dim: i32,
    pub element_size: i32,
    pub property_flags: u64,
    pub rep_index: u16,
    pub blueprint_replication_condition: u8,
    pub offset_internal: u32,
    pub rep_notify_func: FName,
    pub property_link_next: *mut FProperty,
    pub next_ref: *mut FProperty,
    pub destructor_link_next: *mut FProperty,
    pub post_construct_link_next: *mut FProperty,
}

#[allow(non_upper_case_globals)]
static mut FPropertyVTable_ptr: *const FPropertyVTable = core::ptr::null();
