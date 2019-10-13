//! Steam Library

#![allow(non_camel_case_types)]
#![allow(unused)]

use crate::{
    steam_library::*,
};

extern "system" {
    /// dlopen
    pub fn dlopen(filename: *const u8, flag: isize) -> *mut core::ffi::c_void;
    
    /// dlsym
    pub fn dlsym(handle: *mut core::ffi::c_void, symbol: *const u8) -> *mut core::ffi::c_void;

    /// dlclose
    pub fn dlclose(handle: *mut core::ffi::c_void) -> isize;
}

// libsteam_api.so
const STEAMWORKS_SO: [u8; 18] = [
    '.' as u8,
    '/' as u8,
    'l' as u8,
    'i' as u8,
    'b' as u8,
    's' as u8,
    't' as u8,
    'e' as u8,
    'a' as u8,
    'm' as u8,
    '_' as u8,
    'a' as u8,
    'p' as u8,
    'i' as u8,
    '.' as u8,
    's' as u8,
    'o' as u8,
    0,
];

// Library
pub (crate) static mut C_STEAMAPI_SO: *mut core::ffi::c_void = core::ptr::null_mut();

// Library
pub (crate) static mut C_STEAMCLIENT_SO: *mut core::ffi::c_void = core::ptr::null_mut();

pub (crate) fn shutdown_library() {
    unsafe {
        if C_STEAMAPI_SO as usize != 0 {
            dlclose(C_STEAMAPI_SO);
            C_STEAMAPI_SO = core::ptr::null_mut();
        }

        if C_STEAMCLIENT_SO as usize != 0 {
            dlclose(C_STEAMCLIENT_SO);
            C_STEAMCLIENT_SO = core::ptr::null_mut();
        }
    }
}

macro_rules! load_steamapi_function {
    ($name:tt, $static_name:ident) => {
        {
            let ptr = dlsym(
                C_STEAMAPI_SO, 
                format!("{}\0", $name).as_ptr()
            );
            if ptr == core::ptr::null_mut() {
                println!("{}\0", $name);
                println!("failed to load {}", $name);
                shutdown_library();
                return false;
            }
            $static_name = core::mem::transmute(ptr);
        }
    }
}

#[cfg(not(feature = "extremely-lowlevel-steam"))]
#[inline(always)]
fn setup_library_steampi_lowlevel() {}

#[inline(always)]
pub (crate) fn setup_library() -> bool {
    unsafe {
        // Detect clean state.
        if C_STEAMAPI_SO as usize == 0 {
            // RTLD_LAZY
            C_STEAMAPI_SO = dlopen(STEAMWORKS_SO.as_ptr(), 1);
            if C_STEAMAPI_SO as usize == 0 {
                println!("failed to load steamapi (dynamic library)");
                println!("'{}'", String::from_utf8_lossy(&STEAMWORKS_SO));
                return false;
            }

            load_steamapi_function!("SteamAPI_Init", C_STEAMAPI_INIT);
            load_steamapi_function!("SteamAPI_Shutdown", C_STEAMAPI_SHUTDOWN);
            load_steamapi_function!("SteamClient", C_STEAMCLIENT);
            load_steamapi_function!("SteamAPI_ISteamClient_BShutdownIfAllPipesClosed", C_STEAMAPI_ISTEAMCLIENT_BSHUTDOWNIFALLPIPESCLOSED);

            setup_library_steampi_lowlevel();
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

