//! Sublimation Live Wire
//! 
//! Designed to be operated in a spawned/forked process, unless you have
//! a tooling ID for your given application.

#[macro_use]
extern crate neon;

use neon::prelude::*;

// Name + Version (mostly for display)
fn version_string(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(format!("{} ({})",env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION"))))
}

// Version (for functionality testing)
fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(format!("{}",env!("CARGO_PKG_VERSION"))))
}

fn sublimation_init(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    Ok(cx.boolean(sublimation::setup_library()))
}

fn sublimation_startapp(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let f = cx.argument::<JsNumber>(0)?.value();
    std::env::set_var("SteamAppId", format!("{}", f as u64));
    Ok(cx.boolean(sublimation::steam_library::init()))
}

fn sublimation_app_build(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let steam_client = {
        let client = sublimation::steam_library::client::SteamClient::new();
        if client.is_none() {
            panic!("failed to get steam client");
        }
        client.unwrap()
    };                

    let steamapps = sublimation::steam_library::get_steamapps(&steam_client, None);
    if steamapps as usize == 0 {
        panic!("failed to get steamapps");
    }
    Ok(cx.number(sublimation::steam_library::get_application_build(steamapps) as f64))
}

fn sublimation_app_dir(mut cx: FunctionContext) -> JsResult<JsString> {
    let f = cx.argument::<JsNumber>(0)?.value();
    let steam_client = {
        let client = sublimation::steam_library::client::SteamClient::new();
        if client.is_none() {
            panic!("failed to get steam client");
        }
        client.unwrap()
    };                

    let steamapps = sublimation::steam_library::get_steamapps(&steam_client, None);
    if steamapps as usize == 0 {
        panic!("failed to get steamapps");
    }
    let build_dir = sublimation::steam_library::get_application_directory(steamapps, f as u32);
    if build_dir.is_none() {
        panic!("failed to get build_directory");
    }

    let build_dir_string = String::from_utf8(build_dir.unwrap());
    if build_dir_string.is_err() {
        panic!("failed to encode build_directory");
    } else {
        let build_dir = build_dir_string.unwrap();
        let build_path = std::path::Path::new(&build_dir);
        if build_path.exists() {
            Ok(cx.string(build_path.to_str().unwrap()))
        } else {
            // Invalid handling, Steam thinks it's here, so give it a spin.
            panic!("sublimation cannot find path: {}", build_path.to_str().unwrap())
        }
    }
}

register_module!(mut m, {
    m.export_function("sublimation_init", sublimation_init)?;
    m.export_function("sublimation_startapp", sublimation_startapp)?;
    
    m.export_function("sublimation_app_build", sublimation_app_build)?;
    m.export_function("sublimation_app_dir", sublimation_app_dir)?;

    m.export_function("version_string", version_string)?;
    m.export_function("version", version)?;
    Ok(())
});