/*
 * Errepi Net Rust Bindings
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos/cron_bridge.proto");
    println!("cargo:rerun-if-changed=protos/generic_regs.proto");

    // Both protos declare `package protos` and each defines an `AppInfo`
    // message, so they cannot share a prost package. Stage copies in OUT_DIR
    // with distinct packages and compile each separately.
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let cron_staged = stage_proto("protos/cron_bridge.proto", "errepi_cron", &out_dir)?;
    let regs_staged = stage_proto("protos/generic_regs.proto", "errepi_regs", &out_dir)?;

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
