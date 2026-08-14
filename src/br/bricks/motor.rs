use bitflags::bitflags;

use super::brick::*;

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct UMotorBrickFlags: u8
    {
        const MANUALLY_SHIFTED = 0x1;
        const WAS_ON_THROTTLE = 0x2;
        const IS_RUNNING = 0x4;
        const LAST_AUTOMATIC_HAND_BRAKE = 0x8;
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UMotorBrick
{
    pub brick: UBrick,
    pub StartupTime: f32,
    pub CurrentRPM: f32,
    pub ShiftTime: f32,
    pub flags: UMotorBrickFlags,
    pub ThrottleInputChannelValue: f32,
    pub ExhaustThrottle: f32,
    pub CompressorBoostFactor: f32,
}
