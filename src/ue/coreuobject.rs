#![allow(nonstandard_style)]


use brickworks::br_print;
use brickworks::set_module_name;
set_module_name!("coreuobject");

use crate::ue::fframe::FFrame;
use crate::ue::fstring::FString;

use super::fname::*;
use super::uclass::*;

pub(crate) static mut StaticConstructObject_Internal: Option<StaticConstructObject_t> = None;
pub(crate) type StaticConstructObject_t = unsafe extern "C" fn ( params: FStaticConstructObjectParameters) -> *mut UObjectBase;

pub(crate) type fnProcessInternal = unsafe extern "C" fn(obj: *mut UObject, stack: *mut FFrame, result: *mut ());
pub(crate) static mut ProcessInternal_ptr: Option<fnProcessInternal> = None;
pub(crate) static mut ProcessInternal_hook: Option<fnProcessInternal> = None;

pub(crate) type fnStaticLoadObject = unsafe extern "C" fn
( class: *mut UClass, in_outer: *mut UObject, inname: *const u16, filename: *const u16, flags: u32, reconciliation: bool ) -> *mut UObject;
pub(crate) static mut StaticLoadObject_ptr: Option<fnStaticLoadObject> = None;

pub(crate) static mut UCLASS: *mut UClass = core::ptr::null_mut();
pub unsafe fn default_uclass() -> *mut UClass
{
    UCLASS
}

pub unsafe fn StaticLoadObject(
    class: *mut UClass,
    in_outer: *mut UObject,
    inname: *const u16,
    filename: *const u16,
    flags: u32,
    reconciliation: bool
) -> *mut UObject
{
    (StaticLoadObject_ptr.unwrap())(class, in_outer, inname, filename, flags, reconciliation)
}

#[repr(C)]
#[derive(Debug)]
pub struct FUObjectItem
{
    pub object: *mut UObject,
    pub flags: i32,
    pub cluster_root_index: i32,
    pub serial: i32,

}

#[repr(C)]
#[derive(Debug)]
pub struct FChunkedFixedUObjectArray
{
    objects: *mut *mut FUObjectItem,
    pre_allocated_objects: *mut FUObjectItem,
    max_elements: i32,
    num_elements: i32,
    max_chunks: i32,
    num_chunks: i32,
}
impl FChunkedFixedUObjectArray
{
    pub unsafe fn Count(&self) -> i32
    {
        self.num_elements
    }
    pub unsafe fn Get(&self, idx: i32) -> *mut FUObjectItem
    {
        let elements_per_chunk = self.max_elements / self.max_chunks;
        let chunk = idx / elements_per_chunk;
        let within = idx % elements_per_chunk;
        let chunk = *self.objects.add(chunk as usize);
        return chunk.add(within as usize);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FUObjectArray
{
    first_gc_index: i32,
    last_non_gc_index: i32,
    max_not_considered_gc: i32,
    open_disregard: i32,
    pub array: FChunkedFixedUObjectArray
}

pub(crate) static mut GOBJECTS_PTR: *mut FUObjectArray = core::ptr::null_mut();

pub unsafe fn GObjects() -> &'static mut FUObjectArray
{
    return &mut *GOBJECTS_PTR;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FStaticConstructObjectParameters
{
    pub class: *const UClass,
    pub outer: *const UObjectBase,
    pub name: FName,
    pub set_flags: u32,
    pub internal_flags: u32,
    pub copy_transients: bool,
    pub archetype: bool,
    pub template: *const UObjectBase,
    pub instance_graph: *const (),
    pub external_package: *const (),
}

#[repr(C)]
#[derive(Debug, Copy , Clone)]
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
pub struct UObjectVTable
{
    pub Destructor: unsafe extern "C" fn( obj: *mut UObject ),
    pub RegisterDependencies: unsafe extern "C" fn( obj: *mut UObject ),
    pub DeferredRegister: unsafe extern "C" fn( obj: *mut UObject ),
    pub CanBeClusterRoot: unsafe extern "C" fn( obj: *mut UObject ),
    pub CanBeInCluster: unsafe extern "C" fn( obj: *mut UObject ),
    pub CreateCluster: unsafe extern "C" fn( obj: *mut UObject ),
    pub OnClusterMarkedAsPendingKill: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetDetailedInfoInternal: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostInitProperties: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostCDOContruct: unsafe extern "C" fn( obj: *mut UObject ),
    pub PreSaveRoot: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostSaveRoot: unsafe extern "C" fn( obj: *mut UObject ),
    pub PreSave: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsReadyForAsyncPostLoad: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostLoad: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostLoadSubobjects: unsafe extern "C" fn( obj: *mut UObject ),
    pub BeginDestroy: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsReadyForFinishDestroy: unsafe extern "C" fn( obj: *mut UObject ),
    pub FinishDestroy: unsafe extern "C" fn( obj: *mut UObject ),
    pub Serialize__FStructuredArchiveRecord: unsafe extern "C" fn( obj: *mut UObject ),
    pub Serialize__Ref_FArchive: unsafe extern "C" fn( obj: *mut UObject ),
    pub ShutdownAfterError: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostInterpChange: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostRename: unsafe extern "C" fn( obj: *mut UObject ),
    pub PreDuplicate: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostDuplicate__EDuplicateMode_Type: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostDuplicate__bool: unsafe extern "C" fn( obj: *mut UObject ),
    pub NeedsLoadForClient: unsafe extern "C" fn( obj: *mut UObject ),
    pub NeedsLoadForServer: unsafe extern "C" fn( obj: *mut UObject ),
    pub NeedsLoadForTargetPlatform: unsafe extern "C" fn( obj: *mut UObject ),
    pub NeedsLoadForEditorGame: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsEditorOnly: unsafe extern "C" fn( obj: *mut UObject ),
    pub HasNonEditorOnlyReferences: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsPostLoadThreadSafe: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsDestructionThreadSafe: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetPreloadDependencies: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetPrestreamPackages: unsafe extern "C" fn( obj: *mut UObject ),
    pub ExportCustomProperties: unsafe extern "C" fn( obj: *mut UObject ),
    pub ImportCustomProperties: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostEditImport: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostReloadConfig: unsafe extern "C" fn( obj: *mut UObject ),
    pub Rename: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetDesc: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetSparseClassDataStruct: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetWorld: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetNativePropertyValues: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetResourceSizeEx: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetExporterName: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetRestoreForUObjectOverwrite: unsafe extern "C" fn( obj: *mut UObject ),
    pub AreNativePropertiesIdenticalTo: unsafe extern "C" fn( obj: *mut UObject ),
    /* syka blyat */
    pub GetAssetRegistryTags_C__Ref_TArray_UObject_FAssetRegistryTag_TSizedDefaultAllocator_32_: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsAsset: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetPrimaryAssetId: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsLocalizedResource: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsSafeForRootSet: unsafe extern "C" fn( obj: *mut UObject ),
    pub TagSubobjects: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetLifetimeReplicatedProps: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsNameStableForNetworking: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsFullNameStableForNetworking: unsafe extern "C" fn( obj: *mut UObject ),
    pub IsSupportedForNetworking: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetSubobjectsWithStableNamesForNetworking: unsafe extern "C" fn( obj: *mut UObject ),
    pub PreNetReceive: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostNetReceive: unsafe extern "C" fn( obj: *mut UObject ),
    pub PostRepNotifies: unsafe extern "C" fn( obj: *mut UObject ),
    pub PreDestroyFromReplication: unsafe extern "C" fn( obj: *mut UObject ),
    pub BuildSubobjectMapping: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetConfigOverridePlatform: unsafe extern "C" fn( obj: *mut UObject ),
    pub OverridePerObjectConfigSection: unsafe extern "C" fn( obj: *mut UObject ),
    pub ProcessEvent: unsafe extern "C" fn( obj: *mut UObject, f: *mut UFunction, params: *mut () ),
    pub GetFunctionCallspace: unsafe extern "C" fn( obj: *mut UObject ),
    pub CallRemoteFunction: unsafe extern "C" fn( obj: *mut UObject ),
    pub ProcessConsoleExec: unsafe extern "C" fn( obj: *mut UObject ),
    pub RegenerateClass: unsafe extern "C" fn( obj: *mut UObject ),
    pub MarkAsEditorOnlySubobject: unsafe extern "C" fn( obj: *mut UObject ),
    pub CheckDefaultSubobjectsInternal: unsafe extern "C" fn( obj: *mut UObject ),
    pub ValidateGeneratedRepEnums: unsafe extern "C" fn( obj: *mut UObject ),
    pub SetNetPushIdDynamic: unsafe extern "C" fn( obj: *mut UObject ),
    pub GetNetPushIdDynamic: unsafe extern "C" fn( obj: *mut UObject ),
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
    pub unsafe fn CallFunction<T>(&mut self, name: &'static str, params: *mut T )
    {
        let class = self.class_private;
        let f = (*class).FindFunctionByName(name, true);
        let vtable = &*(self.vtable as *mut UObjectVTable);
        (vtable.ProcessEvent)(self, f, params as *mut _);

    }
    pub unsafe fn IsExact(&self, s: &'static str) -> bool
    {
        let mut name = String::new();

        let (ptr, count) = self.name_private.as_sptr(); 
        for i in 0..count
        {
            name.push(*ptr.add(i as usize) as char);
        }

        let mut cls = self.class_private;
        loop 
        {
            let (ptr, count) =(*cls).ustruct.ufield.uobject.name_private.as_sptr(); 
            name.push('.');
            for i in 0..count
            {
                name.push(*ptr.add(i as usize) as char);
            }

            cls = (*cls).ustruct.super_struct as *const UClass;
            if cls.is_null() { break; }
        }
        br_print!("{}", name);

        name == s
    }
    pub unsafe fn IsOuterExact(&self, s: &'static str) -> bool
    {
        let name = self.name_private; 
        let s = FString::from_fname(name);
        br_print!("+  {}", s);
        let mut cls = self.outer_private;
        loop 
        {
            if cls.is_null() { break; }

            let s = FString::from_fname((*cls).name_private);
            br_print!("{}", s);
            cls = (*cls).outer_private;
        }
        false
    }

    pub unsafe fn IsA_FName(&self, s: FName) -> bool
    {
        let mut cls = self.class_private;
        loop 
        {
            if (*cls).ustruct.ufield.uobject.name_private.comparison_index == s.comparison_index 
            {
                return true;
            }

            cls = (*cls).ustruct.super_struct as *const UClass;
            if cls.is_null() { break; }
        }
        false
    }
    pub unsafe fn IsA(&self, s: &'static str) -> bool
    {
        let trimmed = &s[1..];
        let fname = FName::search_str(trimmed);
        let mut cls = self.class_private;
        loop 
        {
            if (*cls).ustruct.ufield.uobject.name_private.comparison_index == fname.comparison_index
            {
                return true;
            }

            cls = (*cls).ustruct.super_struct as *const UClass;
            if cls.is_null() { break; }
        }
        false
    }
    pub unsafe fn IsOuterA(&self, s: &'static str) -> bool
    {
        let trimmed = &s[1..];
        let fname = FName::search_str(trimmed);
        let mut cls = self.outer_private;
        loop 
        {
            if cls.is_null() { break; }
            if (*cls).name_private.comparison_index == fname.comparison_index
            {
                return true;
            }

            cls = (*cls).outer_private;
        }
        false
    }
}
