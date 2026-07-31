use super::tarray::FString;

#[repr(C)]
#[derive(Debug,Copy, Clone, Default)]
pub struct FNameEntryId {
    pub value: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FName {
    pub comparison_index: FNameEntryId,
    pub number: u32,
}

pub static NAME_NONE: FName = FName {
    comparison_index: FNameEntryId { value: 0 },
    number: 0,
};

unsafe extern "C"
{
    pub fn BrickRust_string_to_fname( str: *const u8, text: *mut FName );
    pub fn BrickRust_fname_to_fstring( name: *const FName, string: *mut FString );
}
