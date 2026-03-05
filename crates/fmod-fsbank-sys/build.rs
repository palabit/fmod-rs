use build_rs::{input::*, output::*};
use fmod_build_helper::{fsbank_path, transpile};

#[rustfmt::skip]
static HEADERS: &[(&str, &[(&str, &str)])] = &[
    ("fsbank.h", &[]),
    ("fsbank_errors.h", &[]),
];

fn main() {
    rerun_if_changed("build.rs");

    rerun_if_env_changed("DOCS_RS");
    if std::env::var("DOCS_RS").is_ok() {
        let out_dir = out_dir();
        for (header, _) in HEADERS {
            std::fs::write(out_dir.join(header).with_extension("rs"), "// dummy").unwrap();
        }
        return;
    }

    let (inc, lib) = fsbank_path();

    metadata("inc", &inc);
    metadata("lib", &lib);

    rustc_link_search(&lib);
    rerun_if_changed(&lib);
    rustc_link_lib(&fsbank_obj());

    for (header, extra_fixup) in HEADERS {
        transpile(&inc, header, extra_fixup);
    }
}

fn fsbank_obj() -> String {
    if let Some(obj) = dep_metadata("fsbank", "obj") {
        return obj;
    }

    let vendor = cargo_cfg_target_vendor();
    let arch = cargo_cfg_target_arch();
    let mut obj = "fsbank".to_string();

    if vendor == "pc" && matches!(&*arch, "x86" | "x86_64") {
        obj += "_vc";
    }

    obj
}
