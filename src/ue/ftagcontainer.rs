use crate::ue::{fname::FName, tarray::TArray};


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FGameplayTagContainer
{
    pub gameplay_tags: TArray<FName>, 
    pub parent_tags: TArray<FName>,
}
