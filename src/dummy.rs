use crate::{
    CVOID,
    HSteamPipe,
    HSteamUser,
};

pub (crate) extern "C" fn dummy_unused_nargs_noret() {}
pub (crate) extern "C" fn dummy_unused_nargs_pvoid() -> CVOID { 0 as CVOID }
pub (crate) extern "C" fn dummy_unused_nargs_bool() -> bool { false }
pub (crate) extern "C" fn dummy_interface_fetcher(
    _instance: CVOID,
    _user: HSteamUser,
    _pipe: HSteamPipe,
    _version: *const i8
) -> CVOID { 0 as CVOID }
pub (crate) extern "C" fn dummy_get_steamuser() -> HSteamUser { 0 }
pub (crate) extern "C" fn dummy_get_steampipe() -> HSteamPipe { 0 }