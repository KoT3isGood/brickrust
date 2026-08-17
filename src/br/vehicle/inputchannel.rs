use crate::br::vehicle::inputaxis::EVehicleInputAxis;
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVehicleInputChannel
{
    pub vtable: *mut usize,
    pub InputAxis: EVehicleInputAxis,
    /// Sources of the input channel, could be seats or bricks with output channels
    pub SourceBricks: TArray<()>,
    /// Static value for always on channels
    pub Value: f32,
    /// Index that was used before channels were associated by brick references
    pub ChannelIndex_DEPRECATED: u8,
}
