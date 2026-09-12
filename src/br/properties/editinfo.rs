
use crate::ue::tarray::TArray;
use crate::ue::toptional::TOptional;
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
    pub _a01: usize,
    pub _a02: *mut (),
    pub _a03: FText,
    pub description: FText,
    pub container_objects: TArray<FWeakObjectPtr>,
    pub _a05: FWeakObjectPtr,
    pub is_enabled: bool,
    pub is_read_only: bool,
    pub color_style: EBrickUIBrushStyle,
    pub max_combo_box_list_items: i32,
    pub max_combo_box_items_per_row: i32,
    pub pending_changed_event: TSharedPtr<()>,
    pub orientation_override: TOptional<u8>,
}
