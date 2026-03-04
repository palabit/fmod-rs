use build_rs::{input::*, output::*};
use fmod_build_helper::{fmod_core_path, transpile};
use regex::Regex;
use std::fs;

#[rustfmt::skip]
static HEADERS: &[(&str, &[(&str, &str)])] = &[
    ("fmod.h", &[]),
    ("fmod_codec.h", &[
        (r"pub type FMOD_CODEC_(STATE|WAVEFORMAT).*?\n", ""), // nonopaque
    ]),
    ("fmod_common.h", &[
        (r"FMOD_BUILDNUMBER: ::core::ffi::c_int", "FMOD_BUILDNUMBER: ::core::ffi::c_uint"),
        (r"pub type FMOD_ASYNCREADINFO.*?\n", ""), // nonopaque
    ]),
    ("fmod_dsp.h", &[
        (r"pub type FMOD_(DSP_(STATE|BUFFER_ARRAY)|COMPLEX).*?\n", ""), // nonopaque
    ]),
    ("fmod_dsp_effects.h", &[]),
    ("fmod_errors.h", &[]),
    ("fmod_output.h", &[
        (r"pub type FMOD_OUTPUT_(STATE|OBJECT3DINFO).*?\n", ""), // nonopaque
    ]),
];

fn main() {
    rerun_if_changed("build.rs");

    let (inc, lib) = fmod_core_path();

    metadata("inc", &inc);
    metadata("lib", &lib);

    rustc_link_search(&lib);
    rerun_if_changed(&lib);
    rustc_link_lib(&fmod_obj());
    link_extra();

    for (header, extra_fixup) in HEADERS {
        transpile(&inc, header, extra_fixup);
    }

    if cargo_cfg_target_vendor() == "uwp" {
        transpile(&inc, "fmod_uwp.h", &[]);
    }

    if cargo_cfg_target_os() == "ios" {
        transpile(&inc, "fmod_ios.h", &[]);
    }

    let fmod_common = fs::read_to_string(inc + "/fmod_common.h").unwrap();
    // 0xaaaabbcc -> aaaa = product version, bb = major version, cc = minor version.
    let version_pat = Regex::new(r"#define FMOD_VERSION * 0x(\d{4})(\d{2})(\d{2})").unwrap();
    let captures = version_pat.captures(&fmod_common).unwrap();
    let version = format!("{}.{}.{}", &captures[1], &captures[2], &captures[3]);
    metadata("version", &version);
}

fn fmod_obj() -> String {
    if let Some(obj) = dep_metadata("fmod", "obj") {
        return obj;
    }

    let vendor = cargo_cfg_target_vendor();
    let arch = cargo_cfg_target_arch();
    let profile = profile();
    let atomics = cargo_cfg_target_feature().contains(&"atomics".to_string());
    let mut obj = match (&*arch, &*profile) {
        ("wasm32", "debug") if atomics => "fmodPL",
        ("wasm32", "release") if atomics => "fmodP_reduced",
        ("wasm32", "release") => "fmod_reduced",
        (_, "debug") => "fmodL",
        (_, "release") => "fmod",
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

fn link_extra() {
    if cargo_cfg_target_vendor() == "apple" {
        match &*cargo_cfg_target_os() {
            "ios" | "tvos" | "visionos" => {
                rustc_link_lib_kind("framework", "AudioToolbox");
                rustc_link_lib_kind("framework", "CoreAudio");
            },
            _ => {},
        }
    }
}
