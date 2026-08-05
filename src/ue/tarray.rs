
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

impl<T: Clone> TArray<T>
{
    pub unsafe fn new() -> TArray<T>
    {
        TArray { data: core::ptr::null_mut(), num: 0, max: 0}
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
    }
}
unsafe extern "C"
{
    fn wctomb( mbchar: *mut u8, wchar: u16 );
    fn mbtowc( wchar: *mut u16, mbchar: *const u8, count: usize );
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct FString
{
    pub data: TArray<u16>,
}

impl fmt::Display for FString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe
        {
            for i in 0..self.data.num
            {
                let wc = *self.data.data.add(i as usize);
                let mut c: u8 = b'0';
                wctomb(&mut c, wc);
                let _ = f.write_char(c as char);
            }
            Ok(())

        }
    }
}
unsafe extern "C"
{
    fn strlen( s: *const u8 ) -> usize;
}

impl FString
{
    pub unsafe fn new() -> FString
    {
        return FString { data: TArray::<u16>::new()}
    }
    pub unsafe fn from_str( s: &'static str) -> FString
    {
        let fs = FString { data: TArray::<u16>::init( 0, s.len() as i32 ) };
        for i in 0..s.len()
        {
            let mut wc: u16 = 0u16;
            mbtowc(&mut wc, s.as_ptr().add(i), 1);
            *fs.data.data.add(i) = wc;
        }
        fs
    }
    pub unsafe fn from_fname( s: FName ) -> FString
    {
        let (ptr, l) = s.as_sptr();
        let fs = FString { data: TArray::<u16>::init( 0, l as i32 ) };
        for i in 0..l as usize
        {
            let mut wc: u16 = 0u16;
            mbtowc(&mut wc, ptr.add(i), 1);
            *fs.data.data.add(i) = wc;
        }
        fs
    }
    pub unsafe fn equals_cstr( &self, s: *const u8 ) -> bool
    {
        if strlen(s)+1 != self.data.num as usize {
            return false;
        }
        for i in 0..self.data.num
        {
            br_print!("{}", *s.add( i as usize ) as char);
            let mut wc: u16 = 0u16;
            mbtowc(&mut wc, s.add(i as usize), 1);

            if wc != *self.data.data.add(i as usize)
            {
                return false;
            }

        }

        true
    }
    pub unsafe fn equals_str( &self, s: &'static str ) -> bool
    {
        if s.len() != self.data.num as usize {
            return false;
        }
        for i in 0..self.data.num
        {
            let mut wc: u16 = 0u16;
            mbtowc(&mut wc, s.as_ptr().add(i as usize), 1);

            if wc != *self.data.data.add(i as usize)
            {
                return false;
            }

        }

        true
    }
    pub unsafe fn free( &mut self )
    {
        self.data.free();
    }
}
