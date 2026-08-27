/*
 * Errepi Net Rust Bindings
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

use std::path::{Path, PathBuf};

const PROTO_FILES: [&str; 2] = ["cron_bridge.proto", "generic_regs.proto"];
const GENERATED_DIR: &str = "src/generated";
const GENERATED_MODULES: [&str; 2] = ["errepi_cron.rs", "errepi_regs.rs"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/cron_bridge.proto");
    println!("cargo:rerun-if-changed=protos/generic_regs.proto");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let have_protos = PROTO_FILES
        .iter()
        .all(|file| Path::new("protos").join(file).exists());
    let have_protoc = std::process::Command::new("protoc")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    if have_protos && have_protoc {
        // Development build: compile from the protos submodule. The generated
        // modules are copied into src/generated so that consumers can build
        // without the submodule (cargo does not initialize git submodules for
        // git dependencies). Commit the diff when bumping the submodule.
        compile_protos(&out_dir)?;
        sync_generated(&out_dir)?;
    } else {
        // Consumer build: use the pre-generated modules committed in the
        // repository. No protoc and no submodule required.
        for module in GENERATED_MODULES {
            let source = Path::new(GENERATED_DIR).join(module);
            if !source.exists() {
                panic!("missing pre-generated module: {source:?}");
            }
            std::fs::copy(&source, out_dir.join(module))?;
        }
    }
    Ok(())
}

fn compile_protos(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Both protos declare `package protos` and each defines an `AppInfo`
    // message, so they cannot share a prost package. Stage copies in OUT_DIR
    // with distinct packages and compile each separately.
    let cron_staged = stage_proto("protos/cron_bridge.proto", "errepi_cron", out_dir)?;
    let regs_staged = stage_proto("protos/generic_regs.proto", "errepi_regs", out_dir)?;

    tonic_prost_build::compile_protos(&cron_staged)?;
    tonic_prost_build::compile_protos(&regs_staged)?;
    Ok(())
}

fn stage_proto(source: &str, package: &str, out_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(source)?;
    let staged = content.replacen("package protos;", &format!("package {package};"), 1);
    let dest = out_dir.join(Path::new(source).file_name().unwrap());
    std::fs::write(&dest, staged)?;
    Ok(dest)
}

fn sync_generated(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(GENERATED_DIR)?;
    for module in GENERATED_MODULES {
        std::fs::copy(out_dir.join(module), Path::new(GENERATED_DIR).join(module))?;
    }
    Ok(())
}
