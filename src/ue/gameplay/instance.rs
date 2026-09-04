#![allow(nonstandard_style)]
use crate::ue::coreuobject::{UObject, UObjectVTable};
use crate::ue::fexec::FExec;
use crate::ue::tarray::TArray;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UGameInstanceVTable
{
    pub uobject: UObjectVTable,
    pub HandleOpenCommand: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub HandleDisconnectCommand: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub HandleReconnectCommand: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub HandleTravelCommand: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub GetWorld: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub FinishDestroy: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub Init: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub Shutdown: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub OnWorldChanged: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub StartGameInstance: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub JoinSession1: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub JoinSession2: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub LoadComplete: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub DebugCreatePlayer: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub DebugRemovePlayer: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub CreateInitialPlayer: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub AddLocalPlayer: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub RemoveLocalPlayer: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub HandleDemoPlaybackFailure: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub OnSeamlessTravelDuringReplay: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub StartRecordingReplay: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub StopRecordingReplay: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub PlayReplay: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub PlayReplayPlaylist: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub AddUserToReplay: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub HandleGameNetControlMessage: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub ReceivedNetworkEncryptionToken: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub ReceivedNetworkEncryptionAck: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub PreloadContentForURL: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub CreateGameModeForURL: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub OverrideGameModeClass: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub DelayPendingNetGameTravel: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub GetOnlineSessionClass: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub GetOnlinePlatformName: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub ClientTravelToSession: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub ReturnToMainMenu: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub RegisterReferencedObject: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub UnregisterReferencedObject: unsafe extern "C" fn( obj: *mut UGameInstance ),
    pub OnStart: unsafe extern "C" fn( obj: *mut UGameInstance ),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UGameInstance 
{
    pub uobject: UObject,
    pub exec: FExec,
    pub WorldContext: *mut (),
    pub LocalPlayers: TArray<()>,
    pub OnlineSession: *mut (),
    pub ReferencedObjects: TArray<*mut UObject>,
    pub NotifyPreClientTravelDelegates: TArray<*mut UObject>,
    // TODO
}
