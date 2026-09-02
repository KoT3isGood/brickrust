use crate::{br_print, win32::*};
use crate::BrickRust_print;

#[derive(Debug)]
pub struct Signature {
    pub bytes: &'static [u8],
    pub mask: &'static [bool],
}

#[derive(Debug)]
pub struct ExternalFunction
{
    pub signature: Signature,
    pub ptr_fn: *mut usize
}

/**
 * Searches array of bytes for specific signature
 * */
pub unsafe fn lookup_data( data: *const u8, data_len: usize, sign: Signature ) -> *const u8
{


    let data_len = data_len - sign.bytes.len();
    for i in 0..=data_len
    {
        let mut found = true;
        for j in 0..sign.bytes.len()
        {
            if sign.mask[j] && (*data.add(i+j) != sign.bytes[j])
            {
                found = false;
                break;
            }

        }
        if found {
            return data.add(i);
        }

    }

    return core::ptr::null();
}

/**
 * Searches game for specifc signature
 * */
pub unsafe fn lookup_unsafe( sign: Signature ) -> *const u8
{
    let data_len: usize = get_base_size();
    let data: *const u8 = get_base_address() as *const u8;
    lookup_data(data, data_len, sign)
}

/**
 * Searches game for specifc signature
 * Panics when not found
 * */
pub fn lookup( note: &'static str, sign: Signature ) -> *const u8
{
    let sig = unsafe { lookup_unsafe(sign) };
    if sig.is_null()
    {
        br_print!("{}: {:p}", note, sig);
        panic!("Failed to find {}", note)
    }
    else
    {
        br_print!("{}: {:p}", note, sig);
    }
    sig
}
pub enum LookupMode
{
    SignatureStart,
    Offset32,
    Direct64,
}
/*
pub fn lookup2( note: &'static str, sign: Signature, offset: usize, mode: LookupMode ) -> *const u8
{

}
*/

#[cfg(test)]
mod test
{
    use super::*;
    use brickrust_macros::sig;

    #[test]
    fn lookup_byte()
    {
        const DATA: Signature = sig!("1F 2F 3F FF 1F 2F 4F AB CD EF");
        
        let mut r: *const u8;
        unsafe {
            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("1F"));
            assert_eq!(DATA.bytes.as_ptr(), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("2F"));
            assert_eq!(DATA.bytes.as_ptr().add(1), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("AB"));
            assert_eq!(DATA.bytes.as_ptr().add(7), r);
        }
    }

    #[test]
    fn lookup_bytes()
    {
        const DATA: Signature = sig!("1F 2F 3F FF 1F 2F 4F AB CD EF");
        
        let mut r: *const u8;
        unsafe {
            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("1F 2F 3F FF"));
            assert_eq!(DATA.bytes.as_ptr(), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("2F 3F"));
            assert_eq!(DATA.bytes.as_ptr().add(1), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("AB CD"));
            assert_eq!(DATA.bytes.as_ptr().add(7), r);
        }
    }

    #[test]
    fn lookup_bytes_not_found()
    {
        const DATA: Signature = sig!("1F 2F 3F FF 1F 2F 4F AB CD EF");
        
        let mut r: *const u8;
        unsafe {
            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("1F 2F 3F 4F"));
            assert_eq!(core::ptr::null(), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("3F 3F"));
            assert_eq!(core::ptr::null(), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("AB CD EF 00"));
            assert_eq!(core::ptr::null(), r);
        }
    }

    #[test]
    fn lookup_unknown()
    {
        const DATA: Signature = sig!("1F 2F 3F FF 1F 2F 4F AB CD EF");
        
        let mut r: *const u8;
        unsafe {
            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("1F ?? 3F"));
            assert_eq!(DATA.bytes.as_ptr(), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("1F ?? 4F"));
            assert_eq!(DATA.bytes.as_ptr().add(4), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("?? ?? 1F 2F"));
            assert_eq!(DATA.bytes.as_ptr().add(2), r);

            r = lookup_data(DATA.bytes.as_ptr(), DATA.bytes.len(), sig!("AB ?? ??"));
            assert_eq!(DATA.bytes.as_ptr().add(7), r);
        }
    }
}
