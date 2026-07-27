
use crate::ue::tshared::TWeakPtr;
use crate::ue::ftext::FText;
use crate::br::properties::property::FBrickPropertyInstance;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBrickPropertyEditInfo
{
    pub instance: FBrickPropertyInstance,
    pub this: TWeakPtr<FBrickPropertyEditInfo>,
    pub name: FText,
    pub idk6: *mut (),
    pub idk7: *mut (),
    pub description: FText,
    pub idk8: *mut (),
    pub idk9: *mut (),
    pub idk10: *mut (),
    pub is_enabled: bool,
    pub is_read_only: bool,
    pub color_style: u8,
    pub list_items: i32,
    pub idk12: *mut (),
    pub idk13: *mut (),
    pub idk14: *mut (),
    pub idk15: *mut (),
    pub idk16: *mut (),
    

}
