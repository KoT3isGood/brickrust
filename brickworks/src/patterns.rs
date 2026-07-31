use crate::{br_print, win32::*};

#[derive(Debug)]
pub struct Signature {
    pub bytes: &'static [u8],
    pub mask: &'static [bool],
}

/**
 * find a function by it's signature
 * not really useful for vtables
 * */
pub unsafe fn lookup( sign: Signature ) -> *const u8
{
    let data_len: usize = get_base_size();

    let data: *const u8 = get_base_address() as *const u8;
    let data_len = data_len - sign.bytes.len();
    for i in 0..data_len
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

    br_print!("Failed to find function: {:#?}", sign);
    panic!();
}
