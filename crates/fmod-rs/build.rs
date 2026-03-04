use build_rs::{input::*, output::*};

fn main() {
    rerun_if_changed("build.rs");

    let Some(fmod_version) = dep_metadata("fmod", "version") else {
        return;
    };
    let version = fmod_version.split('.').collect::<Vec<_>>();
    assert!(version.len() == 3);

    let product_version: u8 = version[0].parse().unwrap();
    let major_version: u8 = version[1].parse().unwrap();
    let minor_version: u8 = version[2].parse().unwrap();

    assert_eq!(product_version, 2, "Only FMOD 2.02 and 2.03 are supported");

    rustc_check_cfg_values("fmod_version_major", &["2", "3"]);
    rustc_cfg_value("fmod_version_major", &format!("{major_version}"));

    rustc_check_cfg_values("fmod_has_version_major", &["2", "3"]);
    for version in 2..=major_version {
        rustc_cfg_value("has_fmod_version_major", &format!("{version}"));
    }

    let possible_versions: Vec<_> = (0..=99).map(|v| v.to_string()).collect();
    let possible_versions: Vec<_> = possible_versions.iter().map(|v| v as _).collect();

    rustc_check_cfg_values("fmod_version_minor", &possible_versions[..]);
    rustc_cfg_value("fmod_version_minor", &format!("{minor_version}"));

    rustc_check_cfg_values("fmod_has_version_minor", &possible_versions[..]);
    for version in 0..=minor_version {
        rustc_cfg_value("has_fmod_version_minor", &format!("{version}"));
    }

    rustc_env("FMOD_VERSION", &fmod_version.to_string());
}
