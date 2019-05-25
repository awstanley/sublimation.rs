//! Steam Library

use crate::{
    steam_library::*,
};

type PVOID = crate::CVOID;
type HANDLE = PVOID;
type HMODULE = PVOID;
type BOOL = i32; // int
type FARPROC = HANDLE;
type LPCSTR = *const u8;
type LPCWSTR = *const u16;

//const FALSE: BOOL = 0;
//const TRUE: BOOL = 1;

extern "system" {
    /// [GetProcAddress](https://docs.microsoft.com/en-us/windows/desktop/api/libloaderapi/nf-libloaderapi-getprocaddress)
    /// Retrieves the address of an exported function or variable from the specified dynamic-link library (DLL).
    pub fn GetProcAddress(
        hModule: HMODULE,
        lpProcName: LPCSTR,
    ) -> FARPROC;

    /// [LoadLibraryW](https://docs.microsoft.com/en-us/windows/desktop/api/libloaderapi/nf-libloaderapi-loadlibraryw)
    /// Loads the specified module into the address space of the calling process. The specified module may cause other modules to be loaded.
    /// For additional load options, use the LoadLibraryEx function.
    pub fn LoadLibraryW(
        lpLibFileName: LPCWSTR,
    ) -> HMODULE;

    /// [FreeLibrary](https://docs.microsoft.com/en-us/windows/desktop/api/libloaderapi/nf-libloaderapi-freelibrary)
    /// Frees the loaded dynamic-link library (DLL) module and, if necessary, decrements its reference count. When the reference count reaches zero, the module is unloaded from the address space of the calling process and the handle is no longer valid.
    pub fn FreeLibrary(
        hLibModule: HMODULE,
    ) -> BOOL;
}

#[cfg(target_arch="x86")]
const STEAMWORKS_DLL: [u16; 14] = [
    's' as u16,
    't' as u16,
    'e' as u16,
    'a' as u16,
    'm' as u16,
    '_' as u16,
    'a' as u16,
    'p' as u16,
    'i' as u16,
    '.' as u16,
    'd' as u16,
    'l' as u16,
    'l' as u16,
    0
];

#[cfg(target_arch="x86_64")]
const STEAMWORKS_DLL: [u16; 16] = [
    's' as u16,
    't' as u16,
    'e' as u16,
    'a' as u16,
    'm' as u16,
    '_' as u16,
    'a' as u16,
    'p' as u16,
    'i' as u16,
    '6' as u16,
    '4' as u16,
    '.' as u16,
    'd' as u16,
    'l' as u16,
    'l' as u16,
    0
];

// Library
pub (crate) static mut C_STEAMAPI_DLL: PVOID = 0 as PVOID;

// Library
pub (crate) static mut C_STEAMCLIENT_DLL: PVOID = 0 as PVOID;

pub (crate) fn shutdown_library() {
    unsafe {
        if C_STEAMAPI_DLL as usize != 0 {
            FreeLibrary(C_STEAMAPI_DLL);
            C_STEAMAPI_DLL = 0 as PVOID;
        }

        if C_STEAMCLIENT_DLL as usize != 0 {
            FreeLibrary(C_STEAMCLIENT_DLL);
            C_STEAMCLIENT_DLL = 0 as PVOID;
        }
    }
}

macro_rules! load_steamapi_function {
    ($name:tt, $static_name:ident) => {
        {
            let ptr = GetProcAddress(C_STEAMAPI_DLL, 
                format!("{}\0", $name).as_ptr()
            );
            if ptr == core::ptr::null_mut() {
                println!("failed to load {}", $name);
                shutdown_library();
                return false;
            }
            $static_name = core::mem::transmute(ptr);
        }
    }
}

#[inline(always)]
pub (crate) fn setup_library() -> bool {
    unsafe {

        if C_STEAMAPI_DLL as usize == 0 {
            C_STEAMAPI_DLL = LoadLibraryW(STEAMWORKS_DLL.as_ptr());
            if C_STEAMAPI_DLL as usize == 0 {
                println!("failed to load steamapi");
                return false;
            }

            load_steamapi_function!("SteamAPI_Init", C_STEAMAPI_INIT);
            load_steamapi_function!("SteamAPI_Shutdown", C_STEAMAPI_SHUTDOWN);
            load_steamapi_function!("SteamClient", C_STEAMCLIENT);
            load_steamapi_function!("SteamAPI_ISteamClient_BShutdownIfAllPipesClosed", C_STEAMAPI_ISTEAMCLIENT_BSHUTDOWNIFALLPIPESCLOSED);

            load_steamapi_function!("SteamInternal_CreateInterface", C_STEAMINTERNAL_CREATEINTERFACE);
            load_steamapi_function!("SteamInternal_FindOrCreateUserInterface", C_STEAMINTERNAL_FINDORCREATEUSERINTERFACE);
            load_steamapi_function!("SteamAPI_ReleaseCurrentThreadMemory", C_STEAMAPI_RELEASECURRENTTHREADMEMORY);

            load_steamapi_function!("SteamAPI_ISteamClient_CreateSteamPipe", C_STEAMAPI_ISTEAMCLIENT_CREATESTEAMPIPE);
            load_steamapi_function!("SteamAPI_ISteamClient_BReleaseSteamPipe", C_STEAMAPI_ISTEAMCLIENT_BRELEASESTEAMPIPE);
            load_steamapi_function!("SteamAPI_ISteamClient_ConnectToGlobalUser", C_STEAMAPI_ISTEAMCLIENT_CONNECTTOGLOBALUSER);
            load_steamapi_function!("SteamAPI_ISteamClient_CreateLocalUser", C_STEAMAPI_ISTEAMCLIENT_CREATELOCALUSER);
            load_steamapi_function!("SteamAPI_ISteamClient_ReleaseUser", C_STEAMAPI_ISTEAMCLIENT_RELEASEUSER);
            load_steamapi_function!("SteamAPI_ISteamClient_GetISteamApps", C_STEAMAPI_ISTEAMCLIENT_GETISTEAMAPPS);

            load_steamapi_function!("SteamAPI_GetHSteamUser", C_STEAMAPI_GETHSTEAMUSER);
            load_steamapi_function!("SteamAPI_GetHSteamPipe", C_STEAMAPI_GETHSTEAMPIPE);
            load_steamapi_function!("SteamAPI_ISteamApps_GetAppBuildId", C_STEAMAPI_ISTEAMAPPS_GETAPPBUILDID);
            load_steamapi_function!("SteamAPI_ISteamApps_GetAppInstallDir", C_STEAMAPI_ISTEAMAPPS_GETAPPINSTALLDIR);

            true
        } else {
            true
        }
    }
}

