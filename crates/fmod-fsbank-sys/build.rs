use build_rs::{input::*, output::*};
use fmod_build_helper::{fsbank_path, transpile};

fn main() {
    rerun_if_changed("build.rs");

    let (inc, lib) = fsbank_path();

    metadata("inc", &inc);
    metadata("lib", &lib);

    rustc_link_search(&lib);
    rerun_if_changed(&lib);
    rustc_link_lib(&fsbank_obj());

    transpile(&inc, "fsbank.h", &[]);
    transpile(&inc, "fsbank_errors.h", &[]);
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
