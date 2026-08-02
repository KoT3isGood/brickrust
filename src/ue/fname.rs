use super::tarray::FString;

pub static mut GNAMES_PTR: *mut FNamePool = core::ptr::null_mut();
pub unsafe fn gnames() -> *mut FNamePool
{
    GNAMES_PTR
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FNamePoolFNameEntry
{
    pub key: i16,
    pub name: [u8; 1024]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FNamePoolFNameEntryAllocator
{
    frw_lock: u64,
    pub current_block: i32,
    pub current_block_cursor: i32,
    pub blocks: [*const u8; 8192]
}

#[repr(C)]
#[derive(Debug,Copy, Clone)]
pub struct FNamePool
{
    pub allocator: FNamePoolFNameEntryAllocator,
    pub ansi_count: u32,
    pub wide_count: u32,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
}
impl FName
{
    pub unsafe fn as_cstr(&self) -> *const u8
    {
        let block: u32 = self.comparison_index>>16;
        let block = block as usize;
        let offset = self.comparison_index as u16;
        let offset = offset as usize;
        let entry = (*gnames()).allocator.blocks[block].add(offset*2) as *const FNamePoolFNameEntry; 
        return (*entry).name.as_ptr();
    }
}


pub static NAME_NONE: FName = FName {
    comparison_index: 0,
    number: 0,
};
