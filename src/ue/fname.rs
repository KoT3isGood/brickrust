

use std::{collections::HashMap, ptr::slice_from_raw_parts};

use brickworks::br_print;
use brickworks::patterns::*;

#[cfg(not(feature = "brmk"))]
lookup! {
    pub const GNAMES_PTR: *mut FNamePool = 
        LookupInfo::Binary(5, LookupMode::Offset32, sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
}
#[cfg(feature = "brmk")]
lookup!
{
    pub const GNAMES_PTR: *mut FNamePool = 
        LookupInfo::BinaryDll("BrickRigsModKitSteam-Core.dll", 5, LookupMode::Offset32, sig!("74 09 48 8D 15 ? ? ? ? EB 16"));
}
#[allow(nonstandard_style)]
pub unsafe fn GNames() -> &'static mut FNamePool
{
    &mut **GNAMES_PTR.as_mut_ref()
}

use brickworks::set_module_name;

use crate::ue::fmalloc::{self};
set_module_name!(b"fname\0");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FNameEntry
{
    #[cfg(feature = "brmk")]
    pub comparison_id: u32,
    pub key: u16,
    pub name: [u8; 1024]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FNamePoolAllocator
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
    pub allocator: FNamePoolAllocator,
    pub ansi_count: u32,
    pub wide_count: u32,
}

impl FNamePool
{
    unsafe fn get_entry(&self, ci: u32) -> *const FNameEntry
    {
        let block: u32 = ci>>16;
        let block = block as usize;
        let offset = ci as u16;
        let offset = offset as usize;
        self.allocator.blocks[block].add(offset*FName::NAME_ENTRY_STRIDE) as *const FNameEntry
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FName {
    pub comparison_index: u32,
    #[cfg(feature = "brmk")]
    pub display_index: u32,
    pub number: u32,
}

unsafe extern "C"
{
    fn wctomb( mbchar: *mut u8, wchar: u16 );
    //fn mbtowc( wchar: *mut u16, mbchar: *const u8, count: usize );
}
use core::fmt;
use core::fmt::*;
impl fmt::Display for FName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe
        {
            let (ptr, len, w) = self.as_sptr2();
            if w
            {
                for i in 0..len
                {
                    let wc = *(ptr as *mut u16).add(i as usize);
                    let mut c: u8 = b'0';
                    wctomb(&mut c, wc);
                    let _ = f.write_char(c as char);
                }
            }
            else
            {
                for i in 0..len
                {
                    let c = *ptr.add(i as usize);
                    let _ = f.write_char(c as char);
                }
            }
            Ok(())

        }
    }
}

unsafe extern "C"
{
    pub fn memcmp( l: *const u8, r: *const u8, c: usize ) -> i32;
    pub fn memcpy( l: *const u8, r: *const u8, c: usize ) -> i32;
}


impl FName
{
    pub const BLOCK_SIZE: usize = 2 * 1 << 16;
    #[cfg(feature="brmk")]
    const LENGTH_OFFSET: u8 = 1;
    #[cfg(not(feature = "brmk"))]
    const LENGTH_OFFSET: u8 = 6;
    #[cfg(feature = "brmk")]
    const NAME_ENTRY_HEADER_SIZE: usize = 6;
    #[cfg(not(feature = "brmk"))]
    const NAME_ENTRY_HEADER_SIZE: usize = 2;
    const NAME_ENTRY_STRIDE: usize = align_of::<FNameEntry>();
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
    pub unsafe fn as_sptr(&self) -> (*const u8, u16)
    {
        let entry = GNames().get_entry(self.comparison_index);
        let len = (*entry).key >> FName::LENGTH_OFFSET;
        return ((*entry).name.as_ptr(), len);
    }
    pub unsafe fn as_sptr2(&self) -> (*const u8, u16, bool)
    {
        let entry = GNames().get_entry(self.comparison_index);
        let len = (*entry).key >> FName::LENGTH_OFFSET;
        return ((*entry).name.as_ptr(), len, (*entry).key & 0x1 != 0);
    }

    unsafe fn block_search_str( s: &'static str, block: *const u8, size: usize ) -> Option<FName>
    {
        let mut it = block;
        let start = it; 
        let end = it.add(size).sub(2); 
        while it < end
        {
            let entry = it as *const FNameEntry;

            let len = (*entry).key as u16 >> FName::LENGTH_OFFSET;
            if len == 0 { return None; }
            let len = len as usize;

            let binarylen = if (*entry).key & 0x1 != 0 { len*2 } else {len};
            if it.add(binarylen) > end
            {
                break;
            }

            let binarylen = binarylen+FName::NAME_ENTRY_HEADER_SIZE;
            if (*entry).key & 0x1 != 0
            {
                it = it.add(binarylen);
                it = it.add(it.align_offset(FName::NAME_ENTRY_STRIDE));
                continue;
            }

            if len != s.len() { 
                it = it.add(binarylen);
                it = it.add(it.align_offset(FName::NAME_ENTRY_STRIDE));
                continue 
            }
            let slc = slice_from_raw_parts((*entry).name.as_ptr(), len);
            let _st = str::from_utf8_unchecked(&*slc);
            if memcmp(s.as_ptr(), (*entry).name.as_ptr(), len) == 0
            {
                let idx = (it.offset_from(start) as usize / FName::NAME_ENTRY_STRIDE) as u32;
                return Some(
                    FName { 
                        comparison_index: idx,
                        #[cfg(feature = "brmk")]
                        display_index: idx,
                        number: 0 })
            }
            it = it.add(binarylen);
            it = it.add(it.align_offset(FName::NAME_ENTRY_STRIDE));
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
        let blocks = GNames().allocator.blocks;
        let current_block = GNames().allocator.current_block as usize;
        for i in 0..GNames().allocator.current_block as usize
        {
            let n = FName::block_search_str(s, blocks[i], FName::BLOCK_SIZE );
            if n.is_some()
            {
                let mut n = n.unwrap();
                n.comparison_index |= (i << 16) as u32;
                return n;
            }
        }
        let n = FName::block_search_str(s, blocks[current_block], GNames().allocator.current_block_cursor as usize );
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

        let blocks = GNames().allocator.blocks;
        let current_block = GNames().allocator.current_block as usize;
        let cursor = GNames().allocator.current_block_cursor;
        let name = blocks[current_block].add(cursor as usize) as *mut FNameEntry;
        #[cfg(feature = "brmk")]
        {
            (*name).comparison_id = 0;
        }
        (*name).key = (s.len() as u16) << FName::LENGTH_OFFSET;
        core::ptr::copy_nonoverlapping(s.as_ptr(), &mut (*name).name as *mut u8, s.len());
        let len = FName::NAME_ENTRY_HEADER_SIZE+s.len();
        GNames().allocator.current_block_cursor += len.next_multiple_of(FName::NAME_ENTRY_STRIDE) as i32;

    }

    pub unsafe fn allocate_new_str( s: &'static str ) -> FName
    {
        assert!(s.len()<1024);

        let cursor = GNames().allocator.current_block_cursor;
        let end = cursor as usize + s.len() + 2; 
        let blocks = &mut GNames().allocator.blocks;
        if end > FName::BLOCK_SIZE
        {
            GNames().allocator.current_block_cursor = 0;
            GNames().allocator.current_block += 1;
            let current_block = GNames().allocator.current_block as usize;
            blocks[current_block] = fmalloc::malloc(FName::BLOCK_SIZE) as *const u8;

        }
        let current_block = GNames().allocator.current_block as u32;
        let cursor = GNames().allocator.current_block_cursor as u32;
        FName::write_new_str(s);
        let idx = (current_block<<16)+cursor/(FName::NAME_ENTRY_STRIDE as u32);
        return FName { 
            comparison_index: idx, 
            #[cfg(feature = "brmk")]
            display_index: idx,
            number: 0 };
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
    #[cfg(feature = "brmk")]
    display_index: 0,
    number: 0,
};
