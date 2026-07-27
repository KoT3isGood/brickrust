#![allow(nonstandard_style)]
use core::ops::Deref;

use super::fname::*;
use super::uclass::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum EObjectFlags {
    RF_NoFlags = 0x0000,
    RF_Public = 0x0001,
    RF_Standalone = 0x0002,
    RF_MarkAsNative = 0x0004,
    RF_Transactional = 0x0008,
    RF_ClassDefaultObject = 0x0010,
    RF_ArchetypeObject = 0x0020,
    RF_Transient = 0x0040,
    RF_MarkAsRootSet = 0x0080,
    RF_TagGarbageTemp = 0x0100,
    RF_NeedInitialization = 0x0200,
    RF_NeedLoad = 0x0400,
    RF_KeepForCooker = 0x0800,
    RF_NeedPostLoad = 0x1000,
    RF_NeedPostLoadSubobjects = 0x2000,
    RF_NewerVersionExists = 0x4000,
    RF_BeginDestroyed = 0x8000,
    RF_FinishDestroyed = 0x00010000,
    RF_BeingRegenerated = 0x00020000,
    RF_DefaultSubObject = 0x00040000,
    RF_WasLoaded = 0x00080000,
    RF_TextExportTransient = 0x00100000,
    RF_LoadCompleted = 0x00200000,
    RF_InheritableComponentTemplate = 0x00400000,
    RF_DuplicateTransient = 0x00800000,
    RF_StrongRefOnFrame = 0x01000000,
    RF_NonPIEDuplicateTransient = 0x02000000,
    RF_Dynamic = 0x04000000,
    RF_WillBeLoaded = 0x08000000,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UObjectBaseVTable
{
    pub Destructor: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_1: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_2: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_3: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_4: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_5: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_6: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_7: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_8: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_9: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_10: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_11: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_12: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_13: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_14: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_15: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_16: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_17: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_18: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_19: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_20: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_21: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_22: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_23: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_24: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_25: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_26: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_27: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_28: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_29: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_30: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_31: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_32: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_33: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_34: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_35: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_36: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_37: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_38: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_39: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_40: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_41: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_42: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_43: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_44: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_45: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_46: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_47: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_48: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_49: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_50: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_51: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_52: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_53: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_54: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_55: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_56: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_57: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_58: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_59: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_60: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_61: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_62: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_63: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_64: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_65: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_66: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_67: unsafe extern "C" fn( brick: *mut UObject ),
    pub process_event: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_69: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_70: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_71: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_72: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_73: unsafe extern "C" fn( brick: *mut UObject ),
    pub _a_74: unsafe extern "C" fn( brick: *mut UObject ),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UObjectBase {
    pub vtable: *mut usize,
    pub object_flags: EObjectFlags,
    pub internal_index: i32,
    pub class_private: *const UClass,
    pub name_private: FName,
    pub outer_private: *const UObject,
}


pub type UObject = UObjectBase;

pub trait StaticClass {
    unsafe fn StaticClass() -> *const UStruct;
}
impl UObject
{
    pub unsafe fn IsA<T: StaticClass>(&self) -> bool
    {
        (*self.class_private).ustruct.IsChildOf(T::StaticClass() as *mut UStruct)
    }
}
