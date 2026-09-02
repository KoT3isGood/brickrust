use crate::br::bricks::brick::UBrick;
use crate::ue::coreuobject::{UObject, UObjectVTable};
use crate::ue::ftagcontainer::FGameplayTagContainer;
use crate::ue::ftext::FText;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UBrickEditorObjectStaticInfoVTable
{
    pub uobject: UObjectVTable,
    pub GetObjectTypeDisplayName: unsafe extern "C" fn( obj: *mut UBrickEditorObjectStaticInfo ),
    pub GetDisplayName: unsafe extern "C" fn( obj: *mut UBrickEditorObjectStaticInfo ),
    pub GetBrickEditorFilterTags: unsafe extern "C" fn( obj: *mut UBrickEditorObjectStaticInfo, out_tags: *mut FGameplayTagContainer ),

}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UBrickEditorObjectStaticInfo
{
    pub uobject: UObject,
    pub ObjectClass: *mut UBrick,
    pub ObjectTypeDisplayName: FText,
    pub Description: FText,
    pub Price: f32,
}
