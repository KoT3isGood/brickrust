use crate::ue::tarray::TArray;
use crate::ue::tpair::TPair;
use crate::ue::ftext::FText;
use crate::ue::tshared::TSharedRef;
use crate::ue::ftagcontainer::FGameplayTagContainer;
use crate::br::properties::property::FBrickPropertyInstance;
use crate::br::properties::editinfo::FBrickPropertyEditInfo;

#[repr(C)]
#[derive(Debug)]
pub struct FBrickPropertyReflectionFilter
{
    pub tags: FGameplayTagContainer,

}

#[repr(C)]
#[derive(Debug)]
pub struct FBrickPropertyReflection
{
    pub is_serializing: bool,
    pub container_objects: TArray<()>,
    pub idk: i32,
    pub filter: FBrickPropertyReflectionFilter,
    pub properties: TArray<FBrickPropertyInstance>,
    pub edit_infos: TArray<TPair<TSharedRef<FBrickPropertyEditInfo>, i32>>,
    pub categories: TArray<FText>,
    pub current_category: i32,
    pub parent_property_chain: TArray<()>,
}
