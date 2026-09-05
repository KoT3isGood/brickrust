
use crate::ue::fstring::FString;
use crate::ue::tshared::TSharedRef;
use brickworks::patterns::*;

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

lookup!
{
    pub const Conv_StringToText: unsafe extern "C" fn (a: *const FString) -> FText = 
        LookupInfo::Binary(23, LookupMode::Offset32, sig!("74 41 48 8d 54 24 20 48 8b c8 e8 ?? ?? ?? ?? 48 8b d0 48 8d 4c 24 30"));
}

