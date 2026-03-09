use build_rs::{input::*, output::*};

fn main() {
    rerun_if_changed("build.rs");

    rerun_if_env_changed("DOCS_RS");
    let fmod_version = if std::env::var("DOCS_RS").is_ok() {
        "2.3.12".into() // should match doc-only fmod-rs-sys-for-doc dependency
    } else {
        dep_metadata("fmod", "version").expect("DEP_FMOD_VERSION should be set")
    };

    let version = fmod_version.split('.').collect::<Vec<_>>();
    assert!(version.len() == 3);

    let product_version: u8 = version[0].parse().unwrap();
    let major_version: u8 = version[1].parse().unwrap();
    let minor_version: u8 = version[2].parse().unwrap();

    assert_eq!(product_version, 2, "Only FMOD 2.02 and 2.03 are supported");
    if !matches!(major_version, 2 | 3) {
        warning("Only FMOD 2.02 and 2.03 are supported. Use other versions at your own risk.");
    }

    rustc_check_cfg_values("fmod_version_major", &["2", "3"]);
    rustc_cfg_value("fmod_version_major", &format!("{major_version}"));

    rustc_check_cfg_values("fmod_has_version_major", &["2", "3"]);
    for version in 2..=major_version {
        rustc_cfg_value("fmod_has_version_major", &format!("{version}"));
    }

    let possible_minor_versions: Vec<_> = (0..=99).map(|v| v.to_string()).collect();
    let possible_minor_versions: Vec<_> = possible_minor_versions.iter().map(|v| &**v).collect();

    rustc_check_cfg_values("fmod_version_minor", &possible_minor_versions[..]);
    rustc_cfg_value("fmod_version_minor", &format!("{minor_version}"));

    rustc_check_cfg_values("fmod_has_version_minor", &possible_minor_versions[..]);
    for version in 0..=minor_version {
        rustc_cfg_value("fmod_has_version_minor", &format!("{version}"));
    }

    for major in 2..=3 {
        let possible_versions: Vec<_> = (0..=99)
            .map(|minor| format!("2.{major:02}.{minor:02}"))
            .collect();
        let possible_versions: Vec<_> = possible_versions.iter().map(|v| &**v).collect();
        rustc_check_cfg_values("fmod_has_version", &possible_versions[..]);
        rustc_check_cfg_values("fmod_version", &possible_versions[..]);
    }

    let version = format!("2.{major_version:02}.{minor_version:02}");
    rustc_cfg_value("fmod_version", &version);
    for minor in 0..=minor_version {
        let version = format!("2.{major_version:02}.{minor:02}");
        if major_version == 3 && minor == 9 {
            assert_eq!(version, "2.03.09");
        }
        rustc_cfg_value("fmod_has_version", &version);
    }
    if major_version > 2 {
        for minor in 3..=33 {
            rustc_cfg_value("fmod_has_version", &format!("2.02.{minor:02}"));
        }
    }

    rustc_env("FMOD_VERSION", &version);

    // feature temporarily disabled; please don't complain about it
    rustc_check_cfg_values("feature", &["unstable_extern_type"]);
}
