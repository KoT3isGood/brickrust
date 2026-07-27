use crate::ue::tshared::TSharedRef;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FText
{
    pub data: TSharedRef<()>,
    pub flags: u32,
}
