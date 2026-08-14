
use brickworks::{br_print, set_module_name};
set_module_name!("tarray");

use super::fmalloc;
use core::fmt;
use core::fmt::*;
use super::FName;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TArray<T> {
    pub data: *mut T,
    pub num: i32,
    pub max: i32,
}

pub struct TArrayCounter<T>
{
    pub data: *mut T,
    pub idx: i32,
    pub max: i32,
}

impl <T: Copy> Iterator for TArrayCounter<T>
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.idx >= self.max
        {
            return None;
        }
        let item = unsafe { *self.data.add(self.idx as usize) };
        self.idx+=1;
        return Some(item)
    }
}

impl<T: Clone> TArray<T>
{
    pub unsafe fn new() -> TArray<T>
    {
        TArray { data: core::ptr::null_mut(), num: 0, max: 0}
    }
    pub unsafe fn iter(&self) -> TArrayCounter<T>
    {
        return TArrayCounter { data: self.data, idx: 0, max: self.num }
    }
    pub unsafe fn with_size( num: i32 ) -> TArray<T>
    {
        TArray { data: fmalloc::malloc( size_of::<T>() * num as usize ) as *mut T, num: num, max: num}
    }
    pub unsafe fn init( data: T, num: i32) -> TArray<T>
    {
        let alloc = fmalloc::malloc( size_of::<T>() * num as usize ) as *mut T;
        for i in 0..num
        {
            *alloc.add(i as usize) = data.clone();
        }
        TArray {
            data: alloc,
            num: num,
            max: num,
        }
    }
    pub unsafe fn push( &mut self, data: T ) -> i32 
    {
        if self.num >= self.max
        {
            self.max = u32::next_power_of_two((self.max + 1) as u32) as i32;
            self.data = 
                fmalloc::realloc(
                    self.data as *mut (), 
                    self.max as usize * core::mem::size_of::<T>()
                ) as *mut T
        }
        *self.data.add( self.num as usize ) = data.clone();
        self.num += 1;
        self.num - 1
    }
    #[allow(non_snake_case)]
    pub unsafe fn Add( &mut self, data: T ) -> i32 
    {
        self.push(data)
    }

    pub unsafe fn free( &mut self )
    {
        fmalloc::free(self.data as *mut ());
        self.data = core::ptr::null_mut();
        self.num = 0;
        self.max = 0;
    }
}

