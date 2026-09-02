#![allow(nonstandard_style)]

use crate::ue::coreuobject::UObject;
use crate::ue::delegate::TMulticastDelegate;
use crate::ue::fname::FName;
use crate::ue::fstring::FString;
use crate::ue::gameplay::streamablemgr::FStreamableManager;
use crate::ue::tarray::TArray;
use crate::ue::tmap::TMap;
use crate::ue::tshared::TSharedRef;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UAssetManager
{
    pub uobject: UObject,
    pub AssetPathMap: TMap<FName, ()>,
    pub AssetRuleOverrides: TMap<(), ()>,
    pub ManagementParentMap: TMap<(), TArray<()>>,
    pub CachedAssetBundles: TMap<(), ()>,
    pub AlreadyScannedDirectories: TArray<FString>,
    pub AllAssetSearchRoots: TArray<FString>,
    pub AddedAssetSearchRoots: TArray<FString>,
    pub StreamableManager: FStreamableManager,
    pub PendingChunkInstalls: TArray<()>,
    pub PrimaryAssetEncryptionKeyCache: TMap<(), ()>,
    pub ObjectReferenceList: TArray<*mut UObject>,
    pub bIsGlobalAsyncScanEnvironment: bool,
    pub bShouldGuessTypeAndName: bool,
    pub bShouldUseSynchronousLoad: bool,
    pub bIsLoadingFromPakFiles: bool,
    pub bShouldAcquireMissingChunksOnLoad: bool,
    pub bOnlyCookProductionAssets: bool,
    pub bIsBulkScanning: bool,
    pub bIsPrimaryAssetDirectoryCurrent: bool,
    pub bIsManagementDatabaseCurrent: bool,
    pub bUpdateManagementDatabaseAfterScan: bool,
    pub bIncludeOnlyOnDiskAssets: bool,
    pub bHasCompletedInitialScan: bool,
    pub NumberOfSpawnedNotifications: i32,
    pub PrimaryAssetTypeRedirects: TMap<FName, FName>,
    pub PrimaryAssetIdRedirects: TMap<FString, FString>,
    pub AssetPathRedirects: TMap<FName, FName>,
    pub OnAddedAssetSearchRootDelegate: TMulticastDelegate,
    pub ChunkInstallDelegateHandle: u64,
    pub bOldTemporaryCachingMode: bool,
    pub AssetTypeMap: TMap<FName, TSharedRef<()>>,
}
