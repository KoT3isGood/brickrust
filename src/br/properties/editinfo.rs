
use crate::ue::tarray::TArray;
use crate::ue::tshared::*;
use crate::ue::ftext::FText;
use crate::br::properties::property::FBrickPropertyInstance;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EBrickUIBrushStyle
{
    Default,
    Highlight,
    Positive,
    Negative,
    Neutral,
    EditorOutline,
    GameLogo,
    DeveloperLogo,
    Custom,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FBrickPropertyEditInfo
{
    pub instance: FBrickPropertyInstance,
    pub this: TWeakPtr<FBrickPropertyEditInfo>,
    pub name: FText,
    pub idk6: *mut (),
    pub idk7: *mut (),
    pub description: FText,
    pub idk9: TSharedPtr<()>,
    pub idk10: *mut (),
    pub _a01: TArray<()>,
    pub _a02: FWeakObjectPtr,
    pub is_enabled: bool,
    pub is_read_only: bool,
    pub color_style: EBrickUIBrushStyle,
    pub idk15: *mut (),
    pub pending_changed_event: TSharedPtr<()>,
}
