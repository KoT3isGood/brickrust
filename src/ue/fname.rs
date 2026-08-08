

use std::collections::HashMap;

pub static mut GNAMES_PTR: *mut FNamePool = core::ptr::null_mut();
pub unsafe fn gnames() -> *mut FNamePool
{
    GNAMES_PTR
}

use brickworks::{br_print, set_module_name};
set_module_name!("fname");

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

impl FNamePool
{
    unsafe fn get_entry(&self, ci: u32) -> *const FNamePoolFNameEntry
    {
        let block: u32 = ci>>16;
        let block = block as usize;
        let offset = ci as u16;
        let offset = offset as usize;
        self.allocator.blocks[block].add(offset*2) as *const FNamePoolFNameEntry
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
}

unsafe extern "C"
{
    pub fn memcmp( l: *const u8, r: *const u8, c: usize ) -> i32;
}

const BLOCK_SIZE: usize = 4 * 1 << 16;


impl FName
{
    pub unsafe fn as_sptr(&self) -> (*const u8, i16)
    {
        let entry = (*gnames()).get_entry(self.comparison_index);
        let len = (*entry).key >> 6;
        return ((*entry).name.as_ptr(), len);
    }

    unsafe fn block_search_str( s: &'static str, block: *const u8, size: usize ) -> Option<FName>
    {
        let mut it = block;
        let start = it; 
        let end = it.add(size).sub(2); 
        while it < end
        {
            let entry = it as *const FNamePoolFNameEntry;

            let len = (*entry).key as u16 >> 6;
            if len == 0 { return None; }
            let len = len as usize;

            if len != s.len() { 
                if (*entry).key & 0x1 != 0 { it = it.add(2).add((len as usize) * 2); } 
                else { it = it.add(2).add(len); }
                it = it.add(it.align_offset(2));
                continue 
            }
            if memcmp(s.as_ptr(), (*entry).name.as_ptr(), len) == 0
            {
                let idx = (it.offset_from(start) as usize / 2) as u32;
                return Some(FName { comparison_index: idx, number: 0 })
            }
            if (*entry).key & 0x1 != 0 { it = it.add(2).add((len as usize) * 2); } 
            else { it = it.add(2).add(len); }
            it = it.add(it.align_offset(2));
        }
        None
    }

    /**
     * Finds FName from string
     *
     * Note: really expensive
     * */
    pub unsafe fn search_str_raw( s: &'static str ) -> FName
    {
        let blocks = (*gnames()).allocator.blocks;
        let current_block = (*gnames()).allocator.current_block as usize;
        for i in 0..(*gnames()).allocator.current_block as usize
        {
            let n = FName::block_search_str(s, blocks[i], BLOCK_SIZE );
            if n.is_some()
            {
                let mut n = n.unwrap();
                n.comparison_index |= (i << 16) as u32;
                return n;
            }
        }
        let n = FName::block_search_str(s, blocks[current_block], (*gnames()).allocator.current_block_cursor as usize );
        if n.is_some()
        {
            let mut n = n.unwrap();
            n.comparison_index |= (current_block << 16) as u32;
            return n;
        }
        NAME_NONE
    }
    /**
     * Finds FName from string.
     *
     * This function uses precaching via hashmaps
     * Safety still remains optional.
     * */
    pub unsafe fn search_str( s: &'static str ) -> FName
    {
        static mut MAP: Option<HashMap<&'static str, FName>> = None;
        #[allow(static_mut_refs)]
        if MAP.is_none() { MAP = Some(HashMap::new()); }
        #[allow(static_mut_refs)]
        let map = MAP.as_mut().unwrap();

        let name = map.get(s);
        if name.is_none() {
            let fname = FName::search_str_raw(s);
            if fname.comparison_index == 0 
            {
                return NAME_NONE;
            }
            let name = map.entry(s).or_insert(fname);

            return *name;
        }
        *name.unwrap()
    }
}


pub const NAME_NONE: FName = FName {
    comparison_index: 0,
    number: 0,
};
