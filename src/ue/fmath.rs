#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector2D
{
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FQuat
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FRotator
{
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FColor
{
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

use core::marker::PhantomData;

use super::gameplay::actor::AActor;
use super::tshared::FWeakObjectPtr;

// ---------------------------------------------------------------------------
// Net-quantised vector types (Engine/Classes/Engine/NetSerialization.h)
// ---------------------------------------------------------------------------
// All net-quantised vector types inherit from FVector at the C++ level.
// For FFI we just use the raw layout of FVector (3 × f32).

/// Net-quantised vector (quantisation to 1 decimal place).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector_NetQuantize
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Net-quantised vector (quantisation to 10 decimal places).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector_NetQuantize10
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Net-quantised vector (quantisation to 100 decimal places).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector_NetQuantize100
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Net-quantised normal (stored as normalized FVector).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector_NetQuantizeNormal
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// ---------------------------------------------------------------------------
// FWeakObjectPtr  (CoreUObject/Public/UObject/WeakObjectPtr.h)
// ---------------------------------------------------------------------------

pub use super::tshared::TWeakObjectPtr;

/// Sentinel used for explicit no-init construction.
pub struct EForceInit;

/// Sentinel used for no-init construction.
pub struct ENoInit;

// ---------------------------------------------------------------------------
// FHitResult  (Engine/Classes/Engine/EngineTypes.h)
// ---------------------------------------------------------------------------

/// Structure containing information about one hit of a trace, such as point
/// of impact and surface normal at that point.
#[repr(C)]
pub struct FHitResult
{
    /** Face index we hit (for complex hits with triangle meshes). */
    pub face_index: i32,

    /** 'Time' of impact along trace direction (ranging from 0.0 to 1.0). */
    pub time: f32,

    /** The distance from the TraceStart to the Location in world space. */
    pub distance: f32,

    /** The location in world space where the moving shape would end up. */
    pub location: FVector_NetQuantize,

    /** Location in world space of the actual contact of the trace shape. */
    pub impact_point: FVector_NetQuantize,

    /** Normal of the hit in world space, for the object that was swept. */
    pub normal: FVector_NetQuantizeNormal,

    /** Normal of the hit in world space, for the object that was hit. */
    pub impact_normal: FVector_NetQuantizeNormal,

    /** Start location of the trace. */
    pub trace_start: FVector_NetQuantize,

    /** End location of the trace. */
    pub trace_end: FVector_NetQuantize,

    /** Penetration depth if trace started inside another object. */
    pub penetration_depth: f32,

    /** Extra data about item that was hit (hit primitive specific). */
    pub item: i32,

    /** Index to item that was hit, also hit primitive specific. */
    pub element_index: u8,

    /** Whether this hit was a blocking collision. */
    pub blocking_hit: u8,

    /** Whether the trace started in penetration. */
    pub start_penetrating: u8,

    /** Physical material that was hit. */
    pub phys_material: FWeakObjectPtr,

    /** Actor hit by the trace. */
    pub actor: TWeakObjectPtr<AActor>,

    /** PrimitiveComponent hit by the trace. */
    pub component: FWeakObjectPtr,

    /** Name of bone we hit (for skeletal meshes). */
    pub bone_name: super::fname::FName,

    /** Name of the bone which took part in hit event. */
    pub my_bone_name: super::fname::FName,
}
