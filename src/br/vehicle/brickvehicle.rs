use bitflags::bitflags;

use crate::br::bricks::brick::UBrick;
use crate::ue::fmath::*;
use crate::ue::tarray::TArray;
use crate::ue::gameplay::actor::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EVehiclePinMode
{
    None,
    RootBrick,
    AllBricks,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FViewTargetZoomCache
{
    pub InputRate: f32,
    pub TargetRatio: f32,
    pub CurrentRatio: f32,
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ABrickVehicleFlags1: u32
    {
        /// Whether initial collision should be avoided after constructing the vehicle
        const bAvoidCollisionOnConstruct = 0x1;
        /// Whether the physics are locally authoritative
        const bHasPhysicsAuthority = 0x2;
        /// Flags set to true if a value has been replicated
        const bRestartTransformReplicated = 0x4;
        /// Used to track changes through replication
        const bSavedCanBeDamaged = 0x8;
        /// Whether we are currently in the UnPossessed function
        const bIsInUnPossessed = 0x10;
    }
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ABrickVehicleFlags2: u8
    {
        const bIsHeatSeekingTarget = 0x1;
        const bIsInteracting = 0x2;
        const bInitializedViewRotation = 0x4;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ABrickVehicle
{
    pub actor: AActor,

    pub _a1: usize,
    pub _a2: usize,
    pub _a3: usize,
    pub _a4: usize,
    pub _a5: usize,
    pub _a6: usize,
    pub _a7: usize,
    pub _a8: usize,
    pub _a9: usize,
    pub _a10: usize,
    pub _a11: usize,
    pub _a12: usize,
    pub _a13: usize,
    pub _a14: usize,
    pub _a15: usize,
    pub _a16: usize,
    pub _a17: usize,
    pub _a18: usize,
    pub _a19: usize,
    pub _a20: usize,
    pub _a21: usize,
    pub _a22: usize,
    pub _a23: usize,
    pub _a24: usize,
    pub _a25: usize,
    pub _a26: usize,
    pub _a27: usize,
    pub _a28: usize,
    pub _a29: usize,
    pub _a30: usize,
    pub _a31: usize,
    pub _a32: usize,
    pub _a33: usize,
    pub _a34: usize,
    pub _a35: usize,
    pub _a36: usize,
    pub _a37: usize,
    pub _a38: usize,
    pub _a39: usize,
    pub _a40: usize,
    pub _a41: usize,
    pub _a42: usize,
    pub _a43: usize,
    pub _a44: usize,
    pub _a45: usize,
    pub _a46: usize,
    pub _a47: usize,
    pub _a48: usize,
    pub _a49: usize,
    pub _a50: usize,
    pub _a51: usize,
    pub _a52: usize,
    pub _a53: usize,
    pub _a54: usize,
    pub _a55: usize,
    pub _a56: usize,
    pub _a57: usize,
    pub _a58: usize,
    pub _a59: usize,
    pub _a60: usize,
    pub _a61: usize,
    pub _a62: usize,
    pub _a63: usize,
    pub _a64: usize,
    pub _a65: usize,
    pub _a66: usize,
    pub _a67: usize,
    pub _a68: usize,
    pub _a69: usize,
    pub _a70: usize,
    pub _a71: usize,
    pub _a72: usize,
    pub _a73: usize,
    pub _a74: usize,
    pub _a75: usize,
    pub _a76: usize,
    pub _a77: usize,
    pub _a78: usize,
    pub _a79: usize,
    pub _a80: usize,
    pub _a81: usize,
    pub _a82: usize,
    pub _a83: usize,
    pub _a84: usize,
    pub _a85: usize,
    pub _a86: usize,
    pub _a87: usize,
    pub _a88: usize,
    pub _a89: usize,
    pub _a90: usize,
    pub _a91: usize,
    pub _a92: usize,
    pub _a93: usize,
    pub _a94: usize,
    pub _a95: usize,
    pub _a96: usize,
    pub _a97: usize,
    pub _a98: usize,
    pub _a99: usize,
    pub _a100: usize,
    pub _a101: usize,
    pub _a102: usize,
    pub _a103: usize,
    pub _a104: usize,
    pub _a105: usize,
    pub _a106: usize,
    pub _a107: usize,
    pub _a108: usize,
    pub _a109: usize,
    pub _a110: usize,
    pub _a111: usize,
    pub _a112: f32,
    pub VehicleConstructionState: u8,
    pub ConstructionBrickCounter: i32,
    pub ConstructionStartTime: f32,
    pub flags1: ABrickVehicleFlags1,
    /// Vehicle bounding box (relative to the root), cached for better performance (and because it can be modified by axles)
    pub VehicleBoundsMin: FVector,
    /// Vehicle bounding box (relative to the root), cached for better performance (and because it can be modified by axles)
    pub VehicleBoundsMax: FVector,
    /// Spawn price of this vehicle
    pub VehiclePrice: f32,
    pub LastRepairTime: f32,
    pub LastMovementReplicationTime: f32,
    pub SpawningPlayerState: usize,
    pub OriginalSpawnPoint: usize,
    pub DownloadReplicator: usize,
    /// List of all connections between bricks, active and broken
    pub BrickConnections: TArray<()>,
    /// List of dynamic connections created by this vehicle
    pub DynamicBrickConnections: TArray<()>,
    /// List of dynamic connections involved this vehicle but owned by another one
    pub ExternalBrickConnections: TArray<()>,
    /// Connections that are waiting to be broken, done at once for better performance
    /// NOTE: The array can contain duplicates, it doesn't matter for the UpdateBrickConnections function
    pub BrickConnectionsToBreak: TArray<()>,
    pub _a130: usize,
    pub _a131: usize,
    pub _a132: usize,
    pub _a133: usize,
    pub _a134: usize,
    pub _a135: usize,
    pub _a136: usize,
    pub _a137: usize,
    pub _a138: usize,
    pub _a139: usize,
    pub _a140: usize,
    pub _a141: usize,
    pub _a142: usize,
    pub _a143: usize,
    pub _a144: usize,
    pub _a145: usize,
    pub _a146: usize,
    pub _a147: usize,
    pub _a148: usize,
    pub _a149: usize,
    pub _a150: usize,
    pub _a151: usize,
    pub _a152: usize,
    pub _a153: usize,
    pub _a154: usize,
    pub _a155: usize,
    pub _a156: usize,
    pub _a157: usize,
    pub _a158: usize,
    pub _a159: usize,
    pub _a160: usize,
    pub _a161: usize,
    pub _a162: usize,
    pub _a163: usize,
    pub _a164: usize,
    pub _a165: usize,
    pub _a166: usize,
    pub _a167: usize,
    pub _a168: usize,
    pub _a169: usize,
    pub _a170: usize,
    pub _a171: usize,
    pub _a172: usize,
    pub _a173: usize,
    pub _a174: usize,
    pub _a175: usize,
    pub _a176: usize,
    pub _a177: usize,
    pub _a178: usize,
    pub _a179: usize,
    pub _a180: usize,
    pub _a181: usize,
    pub _a182: usize,
    pub _a183: usize,
    pub _a184: usize,
    pub _a185: usize,
    pub _a186: usize,
    pub _a187: usize,
    pub _a188: usize,
    pub _a189: usize,
    pub _a190: usize,
    pub _a191: usize,
    pub _a192: usize,
    pub _a193: usize,
    pub _a194: usize,
    pub _a195: usize,
    pub _a196: usize,
    pub _a197: usize,
    pub _a198: usize,
    pub _a199: usize,
    pub _a200: usize,
    pub _a201: usize,
    pub _a202: usize,
    pub _a203: usize,
    pub FuelLevelRatio: f32,
    pub RepFuelLevelRatio: u16,
    /// Total fuel capacity
    pub FuelCapacity: f32,
    /// Whether the vehicle is currently pinned in place
    pub PinMode: EVehiclePinMode,
    /// The unique teams of all passengers
    /// NOTE: This has to be replicated since characters can be culled when far away, which means the client would not know about their team affiliation
    pub PassenegerTeamIDs: TArray<()>,	
    pub FuelTanksToExplodeOnClient: TArray<()>,	
    /// Root brick of the vehicle, usually the driver seat
    pub RootBrick: *mut UBrick,
    /// Driver seat of the vehicle
    pub DriverSeat:  *mut UBrick,
    /// List of all cluster root bricks
    pub ClusterRootBricks: TArray<()>,	
    pub ClusterRootBricksWithFluidDynamics: TArray<()>,	
    pub ClusterRootBricksOnFire: TArray<()>,	
    pub ReplicatedBricks: TArray<()>,	
    /// List of seats on this vehicle
    pub SeatBricks: TArray<()>,	
    /// List of camera bricks
    pub CameraBricks: TArray<()>,	
    /// List of all guns
    pub GunBricks: TArray<()>,	
    pub RepMoveClusters: TArray<()>,	
    pub _a204: usize,
    pub _a205: usize,
    pub _a206: usize,
    pub _a207: usize,
    pub _a208: usize,
    pub _a209: usize,
    pub _a210: usize,
    pub _a211: usize,
    pub _a212: usize,
    pub _a213: usize,
    pub _a214: usize,
    pub _a215: usize,
    pub _a216: usize,
    pub flags2: ABrickVehicleFlags2,	
    pub PreInteractionViewRotation: FRotator,
    pub ViewRefLocation: FVector,
    pub ViewRefRotation: FQuat,
    pub ViewRefLinVel: FVector,
    pub ZoomCache: FViewTargetZoomCache,
    pub BricksReplicationKey: u16,
    pub ConnectionsReplicationKey: u16,
    pub FirearmsReplicationKey: u16,
    pub InventoryLoadout: TArray<()>,
    pub VehicleComponent: *mut (),
    pub HandlingAudioComponent: *mut (),
    pub HUDIconComponent: *mut (),
    pub FirearmComponents: *mut TArray<()>,
    pub InventoryComponent: *mut (),
}
