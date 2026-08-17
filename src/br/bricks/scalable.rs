use crate::br::bricks::brick::UBrick;
use crate::ue::fmath::FVector;
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UScalableBrick
{
    pub brick: UBrick,
    /* UScalableBrickBase */
    /* where is it from? */
    pub GeneratedConnectors: TArray<()>,
    pub MaxGeneratedConnectorDist: f32,
    pub ConnectorSpacing: u16,
    /// The lift surface direction
    pub LiftAxisIdx: u8,
    pub BrickSize: FVector,
}
