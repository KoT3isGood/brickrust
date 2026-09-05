use crate::br::bricks::scalable::UScalableBrick;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FFuelTankParams
{
    pub FuelCapacity: f32,
    pub FuelType: *mut (),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UThursterBrick
{
    pub brick: UScalableBrick,
    pub InputChannelValue: f32,
    pub AccumulatedInput: f32,
    pub CurrentThrottle: f32,
    pub GlowBrightness: f32,
    pub AfterglowBrightness: f32,
    pub RepAccumulatedInput: u8,

    // TODO
}
