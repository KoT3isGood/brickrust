use bitflags::bitflags;

use crate::br::vehicle::inputchannel::FVehicleInputChannel;
use crate::ue::tarray::TArray;
use crate::ue::tshared::TSharedPtr;

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
#[derive(Debug, Clone, Copy)]
pub struct UMotorBrick
{
    pub brick: UBrick,
    pub _a0: f32,
    /// Cached list of connected axles
    pub ConnectedAxles: TArray<()>,
    /// World time when the motor has been started
    pub StartupTime: f32,
    /// World time when the last shift has started
    pub ShiftTime: f32,
    /// Current RPM of the rotor
    pub CurrentRPM: f32,
    /// Current throttle input channel value
    pub ThrottleInputChannelValue: f32,
    /// Current throttle used for exhausts
    pub ExhaustThrottle: f32,
    /// The cached boost factor given from all connected compressors
    pub CompressorBoostFactor: f32,
    pub _a1: usize,
    pub StartupAC: TSharedPtr<()>,
    pub MotorAC: TSharedPtr<()>,
    pub BackFireAC: TSharedPtr<()>,
    pub ThrottleInputChannel: FVehicleInputChannel,
    pub GearRatioScale: f32,
    pub bTankDrive: bool,
    pub OnBackFireDelegate: TArray<()>,
}
