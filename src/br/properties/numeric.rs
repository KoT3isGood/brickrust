use super::property::*;
use crate::ue::fmath::FVector;
use crate::ue::fname::FName;
use brickworks::br_print;
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
impl FNumericBrickPropertyBase
{

}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyRange
{
    pub min: FNumericBrickPropertyValue,
    pub max: FNumericBrickPropertyValue,
}



#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyBase
{
    pub property: FBrickProperty,
    pub value_type: TBrickPropAttribute<ENumericValueType>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FNumericBrickPropertyBaseVTable(pub FBrickPropertyVTable);

impl FNumericBrickPropertyBase
{
    unsafe extern "C" fn ExportProperty( prop: *mut FBrickProperty, container: *const ()) -> FString
    {
        let value = FNumericBrickPropertyValue::default();
        todo!()
    }
    pub unsafe extern "C" fn GetTypeName( _prop: *mut FBrickProperty ) -> FName
    {
        let name = FName::default();
        //BrickRust_string_to_fname(b"FNumericBrickProperty GetTypeName\0".as_ptr(), &mut name);
        br_print!("FNumericBrickPropertyBase GetTypeName");
        return name;
    }
    pub unsafe extern "C" fn ComparePropertyValues( _prop: *const FBrickProperty, A: *const (), B: *const () ) -> bool
    {
        let _a = A as *mut f32;
        let _b = B as *mut f32;
        br_print!("FNumericBrickPropertyBase ComparePropertyValues");
        false
    }
    pub unsafe extern "C" fn GetValueAsText( _prop: *mut FBrickProperty, _container: *const (), out_value: *mut FText ) -> bool
    {
        //BrickRust_string_to_ftext(b"10.000\0".as_ptr(), out_value);
        br_print!("FNumericBrickPropertyBase GetValueAsText");
        true
    }
}

#[allow(non_upper_case_globals)]
pub static mut FNumericBrickPropertyBase_ptr: *const FNumericBrickPropertyBaseVTable = core::ptr::null();

impl FNumericBrickPropertyBaseVTable
{
    pub const fn new() -> FNumericBrickPropertyBaseVTable
    {
        let mut vtbl = FBrickPropertyVTable::new(); 
        vtbl.ExportProperty = FNumericBrickPropertyBase::ExportProperty;
        vtbl.GetValueAsText = FNumericBrickPropertyBase::GetValueAsText;
        //vtbl.ComparePropertyValues = FNumericBrickPropertyBase::ComparePropertyValues;
        FNumericBrickPropertyBaseVTable(vtbl)
    }
    pub unsafe fn ptr() -> *const FBrickPropertyVTable
    {
        static TBL: FNumericBrickPropertyBaseVTable = FNumericBrickPropertyBaseVTable::new();
        &TBL.0
    }
}
