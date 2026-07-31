use crate::ue::tshared::TSharedRef;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FText
{
    pub data: TSharedRef<()>,
    pub flags: u32,
}

unsafe extern "C"
{
    pub fn BrickRust_string_to_ftext( str: *const u8, text: *mut FText );
}
