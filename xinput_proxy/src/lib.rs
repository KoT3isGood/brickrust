#![allow(nonstandard_style)]

//!
//! A really simple proxy for xinput1_3.dll, which loads brickworks
//!
//! uses assembly boo!
//!

use core::ffi::*;
type BOOL = i32;
type HANDLE = *mut c_void;
type HINSTANCE = HANDLE;
type HMODULE = HANDLE;
type DWORD = u32;
type LPVOID = *mut c_void;
type LPCSTR = *const u8;

const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;


unsafe extern "system"
{
    fn LoadLibraryA( lib: LPCSTR ) -> HMODULE;
    fn GetProcAddress( lib: HMODULE, proc: LPCSTR ) -> usize; 
}

static mut XInputEnableFn: usize = 0;
static mut XInputGetBatteryInformationFn: usize = 0;
static mut XInputGetCapabilitiesFn: usize = 0;
static mut XInputGetDSoundAudioDeviceGuidsFn: usize = 0;
static mut XInputGetKeystrokeFn: usize = 0;
static mut XInputGetStateFn: usize = 0;
static mut XInputSetStateFn: usize = 0;

#[no_mangle]
unsafe extern "system" fn DllMain(
    _hinstance: HINSTANCE,
    reason: DWORD,
    _reserved: LPVOID,
) -> BOOL
{
    match reason {
        DLL_PROCESS_ATTACH => {
            let xinput = LoadLibraryA(b"C:\\Windows\\System32\\xinput1_3.dll\0".as_ptr());
            XInputEnableFn = GetProcAddress(xinput, b"XInputEnable\0".as_ptr());
            XInputGetBatteryInformationFn = GetProcAddress(xinput, b"XInputGetBatteryInformation\0".as_ptr());
            XInputGetCapabilitiesFn = GetProcAddress(xinput, b"XInputGetCapabilities\0".as_ptr());
            XInputGetDSoundAudioDeviceGuidsFn = GetProcAddress(xinput, b"XInputGetDSoundAudioDeviceGuids\0".as_ptr());
            XInputGetKeystrokeFn = GetProcAddress(xinput, b"XInputGetKeystroke\0".as_ptr());
            XInputGetStateFn = GetProcAddress(xinput, b"XInputGetState\0".as_ptr());
            XInputSetStateFn = GetProcAddress(xinput, b"XInputSetState\0".as_ptr());
            
            
            /* we don't care about the result, it is a cdylib */
            LoadLibraryA(b"brickworks.dll".as_ptr());
        }
        DLL_PROCESS_DETACH => {

        }
        _ =>
        {

        }
        
    }
    1
}

#[cfg(target_arch = "x86_64")]
macro_rules! xinput_proxy {
    ($name:ident, $target:ident) => {
        #[unsafe(naked)]
        #[no_mangle]
        pub(crate) unsafe extern "system" fn $name() {
            core::arch::naked_asm!(
                "jmp qword ptr [rip + {addr}]",
                addr = sym $target,
            );
        }
    };
}

xinput_proxy!(XInputEnable, XInputEnableFn);
xinput_proxy!(XInputGetBatteryInformation, XInputGetBatteryInformationFn);
xinput_proxy!(XInputGetCapabilities, XInputGetCapabilitiesFn);
xinput_proxy!(XInputGetDSoundAudioDeviceGuids, XInputGetDSoundAudioDeviceGuidsFn);
xinput_proxy!(XInputGetKeystroke, XInputGetKeystrokeFn);
xinput_proxy!(XInputGetState, XInputGetStateFn);
xinput_proxy!(XInputSetState, XInputSetStateFn);
