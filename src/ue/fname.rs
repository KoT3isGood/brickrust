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
