use crate::ue::gameplay::actor::AActor;
use crate::ue::tshared::TSharedPtr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AInventoryItem
{
    pub actor: AActor,
    pub StreamableHandle_SkeletalMesh: TSharedPtr<()>,
    pub MID: *mut (),
    pub StaticMeshComponent: *mut (),
    pub SkeletalMeshComponent: *mut (),
    pub InteractionComponent: *mut (),
    pub InventoryComponent: *mut (),
    pub CollisionAudioComponent: *mut (),
    pub StaticInfoClass: *mut (),
}
