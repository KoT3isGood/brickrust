#![allow(nonstandard_style)]

use crate::ue::gcobject::FGCObject;
use crate::ue::fname::FName;
use crate::ue::fstring::FString;
use crate::ue::tarray::TArray;
use crate::ue::tmap::TMap;
use crate::ue::tshared::TSharedRef;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FStreamableManager 
{
    pub gcobject: FGCObject,
    pub StreamableItems: TMap<(), ()>,
    pub StreamableRedirects: TMap<(), ()>,
    pub ManagedActiveHandles: TArray<TSharedRef<()>>,
    pub PendingCombinedHandles: TArray<TSharedRef<()>>,
    pub bForceSynchronousLoads: bool,
    pub ManagerName: FString,
}
