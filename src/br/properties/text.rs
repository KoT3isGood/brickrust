use super::property::*;


#[derive(Debug, Clone, Copy)]
pub struct FTextBrickPropertyVTable(FBrickPropertyVTable);

impl FTextBrickPropertyVTable
{
    pub const fn new() -> FTextBrickPropertyVTable
    {
        FTextBrickPropertyVTable(FBrickPropertyVTable::new())
    }
    pub unsafe fn ptr() -> *const FTextBrickPropertyVTable
    {
        static PTR: FTextBrickPropertyVTable = FTextBrickPropertyVTable::new();
        &PTR
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FTextBrickProperty
{
    pub property: FBrickProperty,
    pub max_text_lenght: i32,
    pub password: bool,
    pub multiline: bool,
    pub user_text: bool
}
