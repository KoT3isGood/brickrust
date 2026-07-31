
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModInfo
{
    pub name: *const u8,
    pub description: *const u8,
    pub version: *const u8,
    pub game_version: *const u8,
    pub authors: *const u8,
}
