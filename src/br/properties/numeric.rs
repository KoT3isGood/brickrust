use super::property::*;
use crate::ue::fmath::FVector;
use crate::ue::fname::FName;
use crate::ue::fstring::FString;
use crate::ue::ftext::FText;
use brickworks::{br_print, lookup, patterns::*};
use crate::BrickRust_print;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ENumericValueType
{
    Float,
    Integer,
    Percent,
    Angle,
    RPM,
    Time,
    Seconds,
    Minutes,
    Hours,
    Days,
    TimeOfDay,
    Money,
    Gear,
    SpeedAuto,
    SpeedMetric,
    SpeedImperial,
    DistanceAuto,
    DistanceMetric,
    DistanceImperial,
    DistanceAutoOrBrickUnits,
    DistanceBrickUnits,
    VolumeAuto,
    VolumeMetric,
    VolumeImperial,
    Bricks,
    MassAuto,
    MassMetric,
    MassImperial,
    ForceAuto,
    ForceMetric,
    ForceImperial,
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EFluAxisLock
{
    None,
    XY,
    XZ,
    YZ,
    All,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FNumericBrickPropertyValue
{
    pub data: FVector,
    pub num_used: u8,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyRange
{
    pub min: FNumericBrickPropertyValue,
    pub max: FNumericBrickPropertyValue,
}

impl FNumericBrickPropertyRange
{
    pub const fn from_f32( min: f32, max: f32 ) -> FNumericBrickPropertyRange
    {
        FNumericBrickPropertyRange
        {
            min: FNumericBrickPropertyValue
            {
                data: FVector { x: min, y: 0.0, z: 0.0 },
                num_used: 1,
            },
            max: FNumericBrickPropertyValue
            {
                data: FVector { x: max, y: 0.0, z: 0.0 },
                num_used: 1,
            }
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyBase
{
    pub property: FBrickProperty,
    pub value_type: TBrickPropAttribute<ENumericValueType>,
    pub _a0: usize,
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
    //pub value_range: TBrickPropAttribute<FNumericBrickPropertyRange>,
    pub axis_lock: TBrickPropAttribute<EFluAxisLock>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyBaseVTable(pub FBrickPropertyVTable);

lookup!
{
    pub const NUMERIC_BRICK_PROPERTY_FLOAT_VTABLE: *const FBrickPropertyVTable = 
        LookupInfo::Binary(-4, LookupMode::Offset32, sig!("66 c7 43 78 00 01 4c 89 bb 80 00 00 00 44 89 bb 88 00 00 00"));
}
