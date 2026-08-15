

use std::{collections::HashMap, ptr::slice_from_raw_parts};

pub static mut GNAMES_PTR: *mut FNamePool = core::ptr::null_mut();
pub unsafe fn gnames() -> *mut FNamePool
{
    GNAMES_PTR
}

use brickworks::{br_print, set_module_name};

use crate::ue::fmalloc::{self, malloc};
set_module_name!(b"fname\0");

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
    pub fn memcpy( l: *const u8, r: *const u8, c: usize ) -> i32;
}

const BLOCK_SIZE: usize = 2 * 1 << 16;

impl FName
{
    pub unsafe fn equals_str(&self, str: &'static str) -> bool
    {
        let (ptr, len) = self.as_sptr();
        if len as usize != str.len() { return false }
        for i in 0..len as usize
        {
            if *ptr.add(i) != str.as_bytes()[i]
            {
                return false;
            }
        }
        return true;
    }
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

            let binarylen = if (*entry).key & 0x1 != 0 { len*2 } else {len};
            if it.add(binarylen) > end
            {
                break;
            }
            let slc = slice_from_raw_parts((*entry).name.as_ptr(), len);
            let st = str::from_utf8_unchecked(&*slc);
            //br_print!("{} {} {}", len, binarylen, st);

            let binarylen = binarylen+2;
            if (*entry).key & 0x1 != 0
            {
                it = it.add(binarylen);
                it = it.add(it.align_offset(2));
                continue;
            }

            if len != s.len() { 
                it = it.add(binarylen);
                it = it.add(it.align_offset(2));
                continue 
            }
            if memcmp(s.as_ptr(), (*entry).name.as_ptr(), len) == 0
            {
                let idx = (it.offset_from(start) as usize / 2) as u32;
                return Some(FName { comparison_index: idx, number: 0 })
            }
            it = it.add(binarylen);
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

    pub unsafe fn write_new_str( s: &'static str )
    {
        assert!(s.len()<1024);

        let blocks = (*gnames()).allocator.blocks;
        let current_block = (*gnames()).allocator.current_block as usize;
        let cursor = (*gnames()).allocator.current_block_cursor;
        let name = blocks[current_block].add(cursor as usize);
        *(name as *mut u16) = (s.len() as u16) << 6;
        core::ptr::copy_nonoverlapping(s.as_ptr(), name.add(2) as *mut u8, s.len());
        (*gnames()).allocator.current_block_cursor += 2 + s.len() as i32;

    }

    pub unsafe fn allocate_new_str( s: &'static str ) -> FName
    {
        assert!(s.len()<1024);

        let cursor = (*gnames()).allocator.current_block_cursor;
        let end = cursor as usize + s.len() + 2; 
        let blocks = &mut (*gnames()).allocator.blocks;
        if end > BLOCK_SIZE
        {
            (*gnames()).allocator.current_block_cursor = 0;
            (*gnames()).allocator.current_block += 1;
            let current_block = (*gnames()).allocator.current_block as usize;
            blocks[current_block] = fmalloc::malloc(BLOCK_SIZE) as *const u8;

        }
        let current_block = (*gnames()).allocator.current_block as u32;
        let cursor = (*gnames()).allocator.current_block_cursor as u32;
        FName::write_new_str(s);
        return FName { comparison_index: (current_block<<16)+cursor/2, number: 0 };
    }

    pub unsafe fn new( s: &'static str ) -> FName
    {
        let name = FName::search_str(s);
        if name.comparison_index == NAME_NONE.comparison_index
        {
            return FName::allocate_new_str(s);
        }
        name
    }
}


pub const NAME_NONE: FName = FName {
    comparison_index: 0,
    number: 0,
};
