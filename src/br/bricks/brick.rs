
use bitflags::bitflags;

use crate::br::vehicle::brickconnection::UBrickConnection;
use crate::ue::coreuobject::*;
use crate::ue::fmath::*;
use crate::ue::tarray::TArray;
use crate::ue::tshared::*;
use crate::ue::uclass::*;
use crate::br::properties::interface::IBrickPropertyInterface;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FBrickTickFunction
{
    pub TickInterval: f32,
    pub flags: u8,
    pub LastTickTime: f32,
    pub Target: *mut UBrick,
}
bitflags! {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct FBrickDamage: u8
    {
        const Burnt = 0x1;
        const Damaged = 0x2;
        const IsOnFire = 0x4;
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UBrickVTable
{
    pub object: UObjectVTable,
    /*
    figure these out
    pub PostLoadBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PostInitializeBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    */
    pub UninitializeBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetBrickEditorObjectDisplayName: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetEditorInterface: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnViewMoveChanged: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnEditorMoveChanged: unsafe extern "C" fn( brick: *mut UBrick ),
    pub CreateEditorParams: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SetupBrickEditorObjectDefaults: unsafe extern "C" fn( brick: *mut UBrick, params: *const () ),
    pub RecycleBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SetupCreateRootComponentParams: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetBrickEditorObjectLocalBounds: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnMirrorBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    pub CalcBrickEditorObjectMass: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetBrickEditorObjectSize: unsafe extern "C" fn( brick: *mut UBrick ),
    pub CalcBrickEditorObjectPrice: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ShouldBeHiddenByViewMode: unsafe extern "C" fn( brick: *mut UBrick ),
    pub UpdateEditorVisualization: unsafe extern "C" fn( brick: *mut UBrick ),
    pub IsBrickPropertyMirroredFrom: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnEditorMoveCommitted: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnEditorMoveCancelled: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnEditorMoveUpdate: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ResolveDeprecatedBrickProperty: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ResolveRemovedBrickProperty: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ShouldTick_Implementation: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetStaticMesh: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetBodySetup: unsafe extern "C" fn( brick: *mut UBrick ),
    pub CalcStaticMeshBounds: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SetupVehicleInventory: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PostConstructVehicle: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnCVarChanged: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ShouldReplicate: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SetupCreateMeshComponentParams: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SetupCreateStaticMeshComponentParams: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetRenderScale3D: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetBodySetupScale3D: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ShouldSpawnCollisionEffects: unsafe extern "C" fn( brick: *mut UBrick ),
    pub SpawnCollisionEffects: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnCalculateMassProperties: unsafe extern "C" fn( brick: *mut UBrick ),

    pub CalcMassPropertiesFromShapes: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetVolumeScale: unsafe extern "C" fn( brick: *mut UBrick ) -> f32,
    pub GetBrickEditorVolumeScale: unsafe extern "C" fn( brick: *mut UBrick ),
    pub ShouldShowGenerateLiftProperty: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetFluidDynamicElements: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetFluidDynamicSurface: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Overridable main tick function
    pub TickBrick: unsafe extern "C" fn( brick: *mut UBrick, delta: f32 ),
    
    /// Whether the brick should currently be ticking
    pub ShouldBrickTick: unsafe extern "C" fn( brick: *mut UBrick ),

    /* UNRELIABLE SECTION, CHECK FIRST */
    /// Get all connectors this brick uses
    pub GetBrickConnectors: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Can be overridden to highlight directions on the connector visualization
    pub GetFocusedConnectorAxisFlags: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Get the relative location and rotation of a connector field
    pub GetConnectorRelativeTransform: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Can be used to create a custom brick connection type
    pub CreateCustomBrickConnection: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Called when a connection involving this brick has been activated
    pub OnBrickConnectionActivated: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Called when a connection involving this brick has broken
    pub OnBrickConnectionBroke: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Called on every child brick during UpdatePartRoot (if bricks were added or removed)
    pub OnUpdatePartRoot: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Called whenever the part root has changed
    pub OnBrickPartRootChanged: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Called whenever bricks have been attached or detached from the cluster (but only while the object is initialized)
    pub OnBricksAddedOrRemovedFromCluster: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Returns the material that should be used for the given slot in the editor
    pub GetEditorBrickMaterial: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Updates the material instance parameters
    pub UpdateBrickMaterial: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Returns the number of material slots on the root component
    pub GetNumMaterialSlots: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Get the current physical material
    pub GetBrickPhysMaterial: unsafe extern "C" fn( brick: *mut UBrick ),
    /* UNRELIABLE SECTION, CHECK FIRST */

    /// Whether this brick can currently be picked up
    pub CanBePickedUp: unsafe extern "C" fn( brick: *mut UBrick ) -> u8,

    /// Whether this brick makes detached parts controllable (RC brick for example)
    pub IsRCBrick: unsafe extern "C" fn( brick: *mut UBrick ) -> bool,
    
    /// Called whenever the controllability state of the cluster has changed (but only while the brick is initialized)
    pub OnIsControllableChanged: unsafe extern "C" fn( brick: *mut UBrick ),

    /* figure the inputs out, they seem to be broken */
    /* prob need to wait for 1.11 BRMK */
    pub HasAnyInputChannel: unsafe extern "C" fn( brick: *mut UBrick ),
    pub GetInputChannels: unsafe extern "C" fn( brick: *mut UBrick ),

    pub MarkBrickBurnt: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnBrickDamageStateChanged: unsafe extern "C" fn( brick: *mut UBrick ),
    pub OnIsFireChanged: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PreRepairBrick: unsafe extern "C" fn( brick: *mut UBrick ),
    pub RepairBrick: unsafe extern "C" fn( brick: *mut UBrick ),

    /// For IBrickContactModifyCallback events forwarded from the vehicle
    pub OnContactModify: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PostContactModify: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Called whenever the brick has taken direct or indirect damage
    pub ReceiveDamageInternal: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Implement for fuel tanks
    pub GetFuelTankRuntimeParams: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Returns the total fuel capacity of the brick
    pub GetFuelCapacity: unsafe extern "C" fn( brick: *mut UBrick ) -> f32,
    /// Returns the fuel type stored in the brick
    pub GetFuelType: unsafe extern "C" fn( brick: *mut UBrick ),
    /// Called whenever the fuel flag has changed
    pub OnHasAnyFuelChanged: unsafe extern "C" fn( brick: *mut UBrick ),

    /// Can be implemented by subclasses to restrict the use of patterns or materials
    pub IsBrickPatternSupported: unsafe extern "C" fn( brick: *mut UBrick, mat: *mut () ) -> u8,
    pub IsBrickMaterialSupported: unsafe extern "C" fn( brick: *mut UBrick, mat: *mut () ) -> u8,
}

bitflags! {

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct UBrickFlags1: u8
    {
	/// Whether the object has been fully initialized, including post initialization
	const bIsInitialized = 0x1;
	/// Whether the object is being initialized right now
	const bIsBeingInitialized = 0x2;
	/// Whether the object is being uninitialized right now
	const bIsBeingUninitialized = 0x4;
        /* from EBrickEditorObjectContext */
	const bIsEditor = 0x8;
	const bIsThumbnailRender = 0x10;
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct UBrickFlags2: u16
    {
	/// Used during UpdateBrickConnections to indicate if the brick's part or cluster root has already been updated
	const bHasPartRootBeenUpdated = 0x1;
	const bHasClusterRootBeenUpdated = 0x2;
	const bHasRepMoveClusterBeenUpdated = 0x4;
	const bIsRepMovePart = 0x8;
	/// Used during UpdateBrickConnections to indicate if children have been added or removed to the part root or cluster root
	const bPartRootChildrenAddedOrRemoved = 0x10;
	const bClusterRootChildrenAddedOrRemoved = 0x20;
	/// Used during UpdateBrickConnections to indicate whether the cluster contains any RC bricks
	const bClusterContainsRCBrick = 0x40;
	/// Whether there are any fluid dynamic elements on this cluster
	const bClusterHasFluidDynamicElements = 0x80;
	/// Whether the part root should currently simulate physics
	const bPartRootSimulatePhysics = 0x100;
	/// Whether the brick is currently connected to the root brick
	const bIsConnectedToRoot = 0x200;
	/// Whether the brick is currently controllable, i.e. connected to the root brick or an RC brick
	const bIsControllable = 0x400;
	/// Whether the brick is connected to any fuel tanks with fuel
	const bHasAnyFuel = 0x800;
	/// Cached flag indicating if the brick has a lift surface
	const bHasFluidDynamicLiftSurface = 0x1000;
	/// Whether we are currently creating the physics state
	const bIsCreatingPhysicsState = 0x2000;
	/// Whether the vehicle and brick are being repaired
	const bIsRepairing = 0x4000;
	/// Whether replication is currently enabled
	const bIsReplicated = 0x8000;
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct UBrickFlags3: u16
    {
	// Whether Blueprint events should be called
	const bEnableBlueprintEvents = 0x1;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UBrick
{
    pub uobject: UObject,
    pub property_interface: IBrickPropertyInterface,
    // The static info this object has been instantiated from
    pub StaticInfoClass: *mut UClass,
    // The unique ID of this object, this is only updated and used before saving
    pub EditorObjectID: u16,
    // Editor only params
    pub EditorParams: *mut (),
    // Whether the object has been fully initialized, including post initialization
    pub flags: UBrickFlags1,
    // Relative spawn location
    pub SpawnLocation: FVector,
    // Relative spawn rotation
    pub SpawnRotation: FRotator,
    // Components that are being used on this object
    pub Components: TArray<TSharedRef<()>>,
    // The root component that manages attachment, collision etc
    pub RootComponent: TSharedPtr<FWeakObjectPtr>,
    /// All connections that involve this brick
    pub Connections: TArray<*mut UBrickConnection>,
    /// Current root of the part this brick belongs to
    pub BrickPartRoot: *mut UBrick,
    /// Current root of the cluster this brick belongs to
    pub BrickClusterRoot: *mut UBrick,
    /// Struct used on part roots to store additional data
    pub PartRootParams: *mut (),
    pub _a0: u32,
    /// While on fire: Time remaining until the brick spreads fire or stops burning
    /// While not on fire: Next time the brick is allowed to catch fire (after being extinguished)
    pub FireTime: f32,
    /// Number of burn intervals the brick yet has to do
    pub NumBurnIntervalsRemaining: u8,
    /// Key used to keep track of replicated changes
    pub ReplicationKey: u16,
    /// Physical material used for this Brick
    pub BrickMaterial: *mut (),
    // Texture pattern for this brick
    pub BrickPattern: *mut (),
    /// Custom color of the brick
    pub BrickColor: FColor,
    pub _a2: u32,
    pub _a3: u32,
    pub _a4: u8,
    pub _a5: u8,
    pub _a6: u8,
    /// Replicated damage information
    pub BrickDamage: FBrickDamage,
    /// Whether this brick should influence fluid dynamics
    pub bGenerateLift: bool,
}

//uclass_game!(UBrick, BrickRigs);
