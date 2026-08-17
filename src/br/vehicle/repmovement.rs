use crate::ue::farrayserializer::FFastArraySerializer;
use crate::ue::tarray::TArray;
use crate::ue::tshared::FWeakObjectPtr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FRepVehicleMovement
{
    pub serializer: FFastArraySerializer,
    pub vehicle: FWeakObjectPtr,
    pub items: TArray<()>,
}
