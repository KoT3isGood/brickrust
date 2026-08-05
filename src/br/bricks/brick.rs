
use crate::ue::coreuobject::*;
use crate::br::properties::interface::IBrickPropertyInterface;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UBrickVTable
{
    pub object: UObjectBaseVTable,
    pub GetNetPushIdDynamic: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PostLoadBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
    pub PostInitializeBrickEditorObject: unsafe extern "C" fn( brick: *mut UBrick ),
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
    pub GetVolumeScale: unsafe extern "C" fn( brick: *mut UBrick ),
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

pub struct UBrick
{
    pub uobject: UObject,
    pub property_interface: IBrickPropertyInterface,
}

//uclass_game!(UBrick, BrickRigs);
