use crate::ue::tarray::TArray;


#[repr(C)]
#[derive(Debug)]
pub struct FGameplayTagContainer
{
    gameplay_tags: TArray<()>, 
    parent_tags: TArray<()>,
}
