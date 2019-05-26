//! Steam Library Wrappers

pub mod client;

use crate::{
    AppId,
    CVOID,
    HSteamUser,
    HSteamPipe,
    dummy::*,
    InterfaceFetcher,
};

// bool SteamAPI_ISteamClient_BShutdownIfAllPipesClosed(client: CVOID)
extern "C" fn dummy_bshutdownifallpipesclosed(_client: CVOID) -> bool { false }
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_BSHUTDOWNIFALLPIPESCLOSED: extern "C" fn(client: CVOID) -> bool = dummy_bshutdownifallpipesclosed;

/// `BShutdownIfAllPipesClosed` proxy
pub fn attempt_shutdown(client: CVOID) -> bool {
    unsafe { C_STEAMAPI_ISTEAMCLIENT_BSHUTDOWNIFALLPIPESCLOSED(client) }
}

// bool SteamAPI_Init();
pub (crate) static mut C_STEAMAPI_INIT: extern "C" fn() -> bool = dummy_unused_nargs_bool;

/// Initialise Steam.
pub fn init() -> bool {
    unsafe { C_STEAMAPI_INIT() }
}

// void SteamAPI_Shutdown
pub (crate) static mut C_STEAMAPI_SHUTDOWN: extern "C" fn() = dummy_unused_nargs_noret;

/// Shuts Steam down.
pub fn shutdown() {
    unsafe { C_STEAMAPI_SHUTDOWN(); }
}

// HSteamPipe SteamAPI_ISteamClient_CreateSteamPipe(intptr_t instancePtr);
extern "C" fn dummy_steamapi_isteamclient_createsteampipe(_client: CVOID) -> HSteamPipe {
    0
}
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_CREATESTEAMPIPE: extern "C" fn(client: CVOID) -> HSteamPipe = dummy_steamapi_isteamclient_createsteampipe;

// bool SteamAPI_ISteamClient_BReleaseSteamPipe(client: CVOID, pipe: HSteamPipe);
extern "C" fn dummy_steamapi_isteamclient_breleasesteampipe(_client: CVOID, _pipe: HSteamPipe) -> bool {
    false
}
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_BRELEASESTEAMPIPE: extern "C" fn(client: CVOID, pipe: HSteamPipe) -> bool = dummy_steamapi_isteamclient_breleasesteampipe;

// HSteamUser SteamAPI_ISteamClient_ConnectToGlobalUser(client: CVOID, pipe: HSteamPipe);
extern "C" fn dummy_steamapi_isteamclient_connecttoglobaluser(_client: CVOID, _pipe: HSteamPipe) -> HSteamUser {
    0
}
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_CONNECTTOGLOBALUSER: extern "C" fn(client: CVOID, pipe: HSteamPipe) -> HSteamUser = dummy_steamapi_isteamclient_connecttoglobaluser;

// SteamAPI_ISteamClient_CreateLocalUser(client: CVOID, phSteamPipe: *const HSteamPipe, eAccountType: isize) -> HSteamUser;
extern "C" fn dummy_steamapi_isteamclient_createlocaluser(_client: CVOID, _pipe: *const HSteamPipe, _account_type: isize) -> HSteamUser {
    0
} 
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_CREATELOCALUSER: extern "C" fn(client: CVOID, pipe: *const HSteamPipe, account_type: isize) -> HSteamUser = dummy_steamapi_isteamclient_createlocaluser;

// void SteamAPI_ISteamClient_ReleaseUser(client: CVOID, pipe: HSteamPipe, user: HSteamUser);
extern "C" fn dummmy_steamapi_isteamclient_releaseuser(_context_init_data: CVOID, _pipe: HSteamPipe, _user: HSteamUser) {}
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_RELEASEUSER: extern "C" fn(context_init_data: CVOID, pipe: HSteamPipe, user: HSteamUser) = dummmy_steamapi_isteamclient_releaseuser;

// void* SteamInternal_CreateInterface( const char *ver );
#[cfg(feature = "extremely-lowlevel-steam")]
extern "C" fn dummy_steaminternal_createinterface(_version: *const u8) -> CVOID {
    0 as CVOID
}
#[cfg(feature = "extremely-lowlevel-steam")]
pub (crate) static mut C_STEAMINTERNAL_CREATEINTERFACE: extern "C" fn(version: *const u8) -> CVOID = dummy_steaminternal_createinterface;

#[cfg(feature = "extremely-lowlevel-steam")]
extern "C" fn dummy_steaminternal_findorcreateuserinterface(_steam_user: HSteamUser, _version: *const u8) -> CVOID {
    0 as CVOID
}
// void* SteamInternal_FindOrCreateUserInterface( HSteamUser hSteamUser, const char *pszVersion );
#[cfg(feature = "extremely-lowlevel-steam")]
pub (crate) static mut C_STEAMINTERNAL_FINDORCREATEUSERINTERFACE: extern "C" fn(steam_user: HSteamUser, _version: *const u8) -> CVOID = dummy_steaminternal_findorcreateuserinterface;

// void SteamAPI_ReleaseCurrentThreadMemory();
pub (crate) static mut C_STEAMAPI_RELEASECURRENTTHREADMEMORY: extern "C" fn() = dummy_unused_nargs_noret;
pub fn release_current_thread_memory() { unsafe { C_STEAMAPI_RELEASECURRENTTHREADMEMORY() }; }

// HSteamUser SteamAPI_GetHSteamUser()
pub (crate) static mut C_STEAMAPI_GETHSTEAMUSER: extern "C" fn() -> HSteamUser = dummy_get_steamuser;

// HSteamPipe SteamAPI_GetHSteamPipe
pub (crate) static mut C_STEAMAPI_GETHSTEAMPIPE: extern "C" fn() -> HSteamPipe = dummy_get_steampipe;

// SteamClient
pub (crate) static mut C_STEAMCLIENT: extern "C" fn() -> CVOID = dummy_unused_nargs_pvoid;

/// Gets a pointer to the SteamClient.
/// `SteamClient()`
pub fn get_steamclient() -> CVOID {
    unsafe { C_STEAMCLIENT() }
}

// ISteamApps* SteamAPI_ISteamClient_GetISteamApps(
//      intptr_t instancePtr,
//      HSteamUser hSteamUser,
//      HSteamPipe hSteamPipe,
//      const char * pchVersion);
pub (crate) static mut C_STEAMAPI_ISTEAMCLIENT_GETISTEAMAPPS: InterfaceFetcher = dummy_interface_fetcher;

#[inline(always)]
pub fn get_steamapps(client: &client::SteamClient, version: Option<*const i8>) -> CVOID {
    unsafe {
        C_STEAMAPI_ISTEAMCLIENT_GETISTEAMAPPS(
            client.instance,
            client.user,
            client.pipe,
            if version.is_some() {
                version.unwrap()
            } else {
                "STEAMAPPS_INTERFACE_VERSION008\0".as_ptr() as *const i8
            }
        )
    }
}

// S_API int SteamAPI_ISteamApps_GetAppBuildId(intptr_t instancePtr);
pub (crate) extern "C" fn dummy_steamapi_isteamapps_getappbuildid(_instance: CVOID) -> isize {
    0
}
pub (crate) static mut C_STEAMAPI_ISTEAMAPPS_GETAPPBUILDID: extern "C" fn(instance: CVOID) -> isize = dummy_steamapi_isteamapps_getappbuildid;
pub fn get_application_build(steamapps: CVOID,) -> isize {
    unsafe { C_STEAMAPI_ISTEAMAPPS_GETAPPBUILDID(steamapps) }
}

// S_API int SteamAPI_ISteamApps_GetAppInstallDir(intptr_t instancePtr, AppId_t nAppID, char * pchDirectory, int cchNameMax);
pub (crate) extern "C" fn dummy_steamapi_isteamapplist_getappinstalldir(_instance: CVOID, _appid: AppId, _directory: *mut u8, _directory_max: u32) -> u32 {
    0
}
pub (crate) static mut C_STEAMAPI_ISTEAMAPPS_GETAPPINSTALLDIR: extern "C" fn (instance: CVOID, appid: AppId, directory: *mut u8, directory_max: u32) -> u32 = dummy_steamapi_isteamapplist_getappinstalldir;

/// Gets the application install directory.
pub fn get_application_directory(steamapps: CVOID, appid: AppId) -> Option<Vec<u8>> {
    let mut buffer = [0u8; 4096];

    let size = unsafe {
        C_STEAMAPI_ISTEAMAPPS_GETAPPINSTALLDIR(
            steamapps,
            appid,
            buffer.as_mut_ptr(),
            4096
        )
    };
    if size == 0 {
        None
    } else {
        Some(Vec::from(&buffer[0..size as usize]))
    }
}

