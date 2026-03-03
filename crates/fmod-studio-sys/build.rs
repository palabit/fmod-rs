use build_rs::{input::*, output::*};
use fmod_build_helper::{fmod_studio_path, transpile};

fn main() {
    rerun_if_changed("build.rs");

    let (inc, lib) = fmod_studio_path();

    metadata("inc", &inc);
    metadata("lib", &lib);

    rustc_link_search(&lib);
    rerun_if_changed(&lib);
    rustc_link_lib(&fmodstudio_obj());

    transpile(&inc, "fmod_studio.h", &[]);
    transpile(&inc, "fmod_studio_common.h", &[]);
}

fn fmodstudio_obj() -> String {
    if let Some(obj) = dep_metadata("fmodstudio", "obj") {
        return obj;
    }

    let vendor = cargo_cfg_target_vendor();
    let arch = cargo_cfg_target_arch();
    let profile = profile();
    let atomics = cargo_cfg_target_feature().contains(&"atomics".to_string());
    let mut obj = match (&*arch, &*profile) {
        ("wasm32", "debug") if atomics => "fmodstudioPL",
        ("wasm32", "release") if atomics => "fmodstudioP",
        (_, "debug") => "fmodstudioL",
        (_, "release") => "fmodstudio",
        _ => unreachable!("unexpected $PROFILE"),
    }
    .to_string();

    if vendor == "pc" && matches!(&*arch, "x86" | "x86_64") {
        obj += "_vc";
    }

    if vendor == "apple" {
        let sim = if cargo_cfg_target_abi().as_deref() == Some("sim") {
            "simulator"
        } else {
            "os"
        };
        match &*cargo_cfg_target_os() {
            "ios" => obj = obj + "_iphone" + sim,
            "tvos" => obj = obj + "_appletv" + sim,
            "visionos" => obj = obj + "_xr" + sim,
            _ => {},
        }
    }

    obj
}
