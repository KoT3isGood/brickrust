use brickworks::br_print;
use crate::BrickRust_print;

use crate::ue::fstring::FString;
use crate::ue::tshared::TSharedRef;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FText
{
    pub data: TSharedRef<()>,
    pub flags: u32,
    pub unk: u32,
}

impl FText
{
    pub unsafe fn from_fstring( s: *const FString) -> FText
    {
        (Conv_StringToText.unwrap())(s)
    }
}

pub(crate) static mut Conv_StringToText: Option<unsafe extern "C" fn (a: *const FString) -> FText> = None;

unsafe extern "C"
{
    pub fn BrickRust_string_to_ftext( str: *const u8, text: *mut FText );
}
