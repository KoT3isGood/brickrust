pub use super::ffield::FField;
pub use super::fname::FName;

#[repr(C)]
#[derive(Debug)]
pub struct FPropertyVTable
{

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
