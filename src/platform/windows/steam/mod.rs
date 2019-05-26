//! Steam Library

#![allow(non_camel_case_types)]
#![allow(unused)]

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

// Registry
type DWORD = u32;
type HKEY = PVOID;
type ACCESS_MASK = DWORD;
type REGSAM = ACCESS_MASK;
type PHKEY = *mut HKEY;
type LONG = i32;
type LPDWORD = *mut DWORD;
type BYTE = u8;
type LPBYTE = *mut BYTE;

// DLL Directory work
type PCWSTR = *const u16;
type DLL_DIRECTORY_COOKIE = PVOID;

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

    // Advapi32.dll ...

    /// [RegOpenKeyExA](https://docs.microsoft.com/en-us/windows/desktop/api/winreg/nf-winreg-regopenkeyexa): Opens the specified registry key. Note that key names are not case sensitive.
    /// To perform transacted registry operations on a key, call the RegOpenKeyTransacted function.
    #[cfg(feature = "extremely-lowlevel-steam")]
    pub fn RegOpenKeyExA(
        hKey: HKEY, 
        lpSubKey: LPCSTR, 
        ulOptions: DWORD, 
        samDesired: REGSAM, 
        phkResult: PHKEY
    ) -> LONG;

    
    /// [RegQueryValueExA](https://docs.microsoft.com/en-us/windows/desktop/api/winreg/nf-winreg-regqueryvalueexa): Retrieves the type and data for the specified value name associated with an open registry key.
    /// To ensure that any string values (REG_SZ, REG_MULTI_SZ, and REG_EXPAND_SZ) returned are null-terminated, use the RegGetValue function.
    #[cfg(feature = "extremely-lowlevel-steam")]
    pub fn RegQueryValueExA(
        hKey: HKEY, 
        lpValueName: LPCSTR, 
        lpReserved: LPDWORD, 
        lpType: LPDWORD, 
        lpData: LPBYTE, 
        lpcbData: LPDWORD
    ) -> LONG;

    
    /// [RegCloseKey](https://docs.microsoft.com/en-us/windows/desktop/api/winreg/nf-winreg-regclosekey): Closes a handle to the specified registry key.
    #[cfg(feature = "extremely-lowlevel-steam")]
    pub fn RegCloseKey(hKey: HKEY) -> LONG;
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

#[cfg(feature = "extremely-lowlevel-steam")]
#[inline(always)]
pub (crate) unsafe fn registry_dll_fix() -> Option<String> {
    let mut registry_key: HKEY = 0 as HKEY;
    let open_key = RegOpenKeyExA(
        2147483649 as HKEY, // HKEY_CURRENT_USER
        "Software\\Valve\\Steam\0".as_ptr(),
        0,
        1,
        &mut registry_key as PHKEY
    );
    
    if open_key == 0 {
        let mut buffer = [0u8; 4096];
        let mut buffer_length = 4096 as DWORD;
        RegQueryValueExA(
            registry_key,
            "SteamPath\0".as_ptr(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            buffer.as_mut_ptr(),
            &mut buffer_length as *mut _
        );
        RegCloseKey(registry_key);

        let output = String::from_utf8(Vec::from_raw_parts(
            buffer.as_mut_ptr(),
            buffer_length as usize,
            buffer.len()
        ));
        
        if output.is_err() {
            None
        } else {
            Some(output)
        }
    } else {
        println!("regsitry fix result: {}", open_key);
        None
    }
}

#[cfg(feature = "extremely-lowlevel-steam")]
#[inline(always)]
fn setup_library_client() -> bool {
    let root = {
        let root = registry_dll_fix();
        if root.is_none() {
            panic!("critical failure p0");
        }
        root.unwrap()
    };

    let mut dll_path: Vec<u16> = root.encode_utf16().collect();
    dll_path.extend_from_slice(&STEAMCLIENT_DLL[..]);
    C_STEAMCLIENT_DLL = LoadLibraryW(dll_path.as_ptr());
    if C_STEAMCLIENT_DLL as usize == 0 {
        println!("failed to load steamclient");
        return false;
    }
}

#[cfg(not(feature = "extremely-lowlevel-steam"))]
#[inline(always)]
fn setup_library_steampi_lowlevel() {}

#[cfg(feature = "extremely-lowlevel-steam")]
#[inline(always)]
fn setup_library_steampi_lowlevel() {
    load_steamapi_function!("SteamInternal_CreateInterface", C_STEAMINTERNAL_CREATEINTERFACE);
    load_steamapi_function!("SteamInternal_FindOrCreateUserInterface", C_STEAMINTERNAL_FINDORCREATEUSERINTERFACE);
}

#[inline(always)]
pub (crate) fn setup_library() -> bool {
    unsafe {
        // Detect clean state.
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

