//! Quick and dirty Steam related code for modding.

#[cfg(target_os = "windows")]
#[path="platform/windows/steam/mod.rs"]
pub (crate) mod platform_library;

pub (crate) type CVOID = *const core::ffi::c_void;
pub (crate) mod dummy;
pub (crate) type InterfaceFetcher = extern "C" fn(instance: *const core::ffi::c_void, user: HSteamUser, pipe: HSteamPipe, version: *const i8) -> CVOID;

/// HSteamUser
pub type HSteamUser = i32;

/// HSteamPipe
pub type HSteamPipe = i32;

/// AppId
pub type AppId = u32;

pub mod steam_library;

/// Sets the library up.
pub fn setup_library() -> bool {
    platform_library::setup_library()
}

#[cfg(test)]
mod test {
    #[test]
    fn init_test() {
        use std::env;
        use super::*;

        let test_app_id_0: AppId = 236870;
        let test_app_id_1: AppId = 863550;

        if !setup_library() {
            panic!("failed to setup library");
        }

        fn inner_test(test_app_id: AppId, terminal: bool) {
            env::set_var("SteamAppId", format!("{}", test_app_id));
            if !steam_library::init() {
                panic!("failed to init steam");
            }

            let mut steam_client = steam_library::client::SteamClient::new()
                .expect("failed to get client");


            let steamapps = steam_library::get_steamapps(&steam_client, None);
            if steamapps as usize == 0 {
                panic!("failed: steamapps is 0");
            }

            let build_dir = steam_library::get_application_directory(steamapps, test_app_id);
            if build_dir.is_none() {
                panic!("build directory failed");
            }

            println!("\tBuild directory:\n\t\t'{}'",
                String::from_utf8(build_dir.unwrap())
                    .expect("failed to make build directory friendly")
                    .as_str()
            );

            println!("\tBuild:\n\t\t'{}'",
                steam_library::get_application_build(steamapps)
            );

            if terminal {
                //steam_client.shutdown();
                env::set_var("SteamAppId", "");
                steam_library::shutdown();
                //steam_library::attempt_shutdown(steam_library::get_steamclient());
            } else {
                env::set_var("SteamAppId", "");
                steam_library::shutdown();
            }
        }

        inner_test(test_app_id_0, false);
        inner_test(test_app_id_1, true);

        steam_library::release_current_thread_memory();

        std::thread::sleep(std::time::Duration::from_millis(2500));
    }
}