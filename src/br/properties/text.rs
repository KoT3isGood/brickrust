use super::property::*;


#[derive(Debug, Clone, Copy)]
pub struct FTextBrickPropertyVTable(pub FBrickPropertyVTable);

impl FTextBrickPropertyVTable
{
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
