use bitflags::bitflags;

use crate::br::bricks::brick::UBrick; 
use crate::br::vehicle::brickconnection::UBrickConnection;
use crate::br::vehicle::inputchannel::FVehicleInputChannel;
use crate::ue::tarray::TArray;
use crate::ue::tshared::{FWeakObjectPtr, TSharedPtr};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FActuatorState
{
    /// Current actuation ratio, -1:1
    pub Actuation: f32,
    /// Current input value, -1:1
    pub Input: f32,
    /// Server time this state was created at
    pub OwnerTimestamp: f32,
    /// Used for the cycle drive mode, indicates that the drive direction is currently inverted
    pub bInvertCycleDirection: bool,
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EActuatorMode
{
    Accumulated,
    Seeking,
    Cycle,
    PhysicsDriven,
    Static,
    Spring
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct UActuatorBrickFlags: u8
    {
	// Cached flag indicating whether the actuator should be replicated
        const bReplicateActuator = 0x1;
	// Whether the local player has authority over the actuator
        const bHasActuatorAuthority = 0x1;
	// Whether the actuator state has been replicated before
        const bActuatorStateReplicated = 0x1;

    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UActuatorBrick
{
    pub brick: UBrick,
    // The current state of the actuator
    pub LocalActuatorState: FActuatorState,
    // Last replicated state of the actuator
    pub RepActuatorState: FActuatorState,
    // Used to smooth actuation replication
    pub _a0: usize,
    pub _a1: usize,
    pub _a2: usize,
    pub _a3: usize,
    pub _a4: usize,
    pub _a5: usize,
    // Actuation value currently used
    pub CurrentActuation: f32,
    // Delta between the replicated and local actuation
    pub RepActuationError: f32,
    // Time remaining over which to smooth out the actuation error
    pub ActuationErrorSmoothTime: f32,
    // Current speed ratio of the actuator, used for sound
    pub ActuationSpeedRatio: f32,
    // All created actuator connections
    pub ActuatorConnections: TArray<FWeakObjectPtr>,
    // Audio component for the movement sound
    pub ActuatorAC: TSharedPtr<()>,
    // Optional interaction component used to actuate via interaction
    pub InteractionComponent: TSharedPtr<()>,
    // The player currently applying interaction input
    pub InteractingPC: *mut (),
    // Current input channel value
    pub InputChannelValue: f32,
    // Accumulated interaction input
    pub InteractionInput: f32,
    // Last time the player has interacted
    pub LastInteractionTime: f32,
    // TODO: max it a FFloatInterval
    // Min and max actuation
    pub ActuationRange_min: f32,
    pub ActuationRange_max: f32,
    pub flags: UActuatorBrickFlags,
    pub ActuatorMode: EActuatorMode,
    pub InputChannel: FVehicleInputChannel,
    pub SpeedFactor: f32,
    pub MaxLimit: f32,
    pub MinLimit: f32,
}
