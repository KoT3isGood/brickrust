use bitflags::bitflags;

use crate::ue::coreuobject::*;
use crate::br::bricks::brick::*;
use crate::br::vehicle::brickvehicle::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EConnectorType {
    None,
    Default,
    Wheel,
    SphereCoupling,
    Hinge,
    Rotor,
    Muzzle,
    LauncherMuzzle,
    FlatRadialHinge,
    BigHinge,
    Piston,
    LinearCoupling,
    Turntable,
    FlatLinearActuator,
    SmallHinge,
    SmallMuzzle,
    VerticalHinge,
    Hinge3x2,
    SmallTurntable,
    LargeTurntable,
    LargePiston,
    SmallFlatLinearActuator,
    MediumHinge,
    TinyTurntable,
    TinyHinge,
    TinyLinearActuator,
    TinySphereCoupling,
    TinyPiston,
    FlareMuzzle,
    MAX,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FBrickConnectionParams
{
    pub Type: EConnectorType,
    pub NumSubConnectors: u16,
    pub Brick0ConnectorIndex: u16,
    pub Brick1ConnectorIndex: u16,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct UBrickConnectionFlags: u8
    {
        const NONE = 0x0;
        const INITIALIZED = 0x01;
        const PHYSICS_CONSTRAINT = 0x02;
        const ACTIVE = 0x04;
        const BROKEN = 0x08;
        const DYNAMIC = 0x10;
        const PENDING_BREAK = 0x20;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UBrickConnection
{
    pub uobject: UObjectBase,
    pub ConnectionIndex: u16,
    pub Brick0: *mut UBrick,
    pub Brick1: *mut UBrick,
    pub BrickID0: u16,
    pub BrickID1: u16,
    pub OtherVehicle: *mut ABrickVehicle,
    pub Params: FBrickConnectionParams,
    pub BreakingDamage: f32,
    pub flags: UBrickConnectionFlags,

}
