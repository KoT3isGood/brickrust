use crate::ue::gameplay::assetmanager::UAssetManager;
use crate::ue::tarray::TArray;
use crate::ue::tmap::TMap;
use crate::ue::uclass::UClass;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ELoadAssetLibrariesMode
{
    Scan,
    Load,
    Unload
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UBrickAssetManager
{
    pub uassetmgr: UAssetManager,
    pub LoadMode: ELoadAssetLibrariesMode,
    pub ModHookClasses: TArray<*mut UClass>,
    pub _a0: TArray<*mut UClass>,
    pub _a1: TArray<*mut UClass>,
    pub BrickStaticInfoClasses: TArray<*mut UClass>,
    pub BrickFilterClasses: TArray<*mut UClass>,
    pub BrickMaterials: TArray<*mut UClass>,
    pub BrickPatternClasses: TArray<*mut UClass>,
    pub BrickDecals: TArray<*mut UClass>,
    pub BrickFonts: TArray<*mut UClass>,
    pub ExhaustEffects: TArray<*mut UClass>,
    pub SirenSequenceClasses: TArray<*mut UClass>,
    pub SirenTypeClasses: TArray<*mut UClass>,
    pub ExplosiveMaterialClasses: TArray<*mut UClass>,
    pub InventoryItemClasses: TArray<*mut UClass>,
    pub UIStyleClasses: TArray<*mut UClass>,
    pub WeatherConditions: TArray<*mut ()>,
    pub LevelInfos: TArray<*mut ()>,
    pub GameModeInfos: TArray<*mut ()>,
    pub BillboardImages: TArray<*mut ()>,
    pub MenuMusicAssets: TArray<*mut ()>,
    pub MenuSequences: TMap<(), ()>,
    pub PropertyWidgetClasses: TArray<()>,
    pub StructPropertyWidgetClass: *mut (),
    pub GenericPropertyWidgetClass: *mut (),
}
