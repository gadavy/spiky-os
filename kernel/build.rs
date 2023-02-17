use std::process::Command;

fn main() {
    // set by cargo, build scripts should use this directory for output files
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let status = Command::new("nasm")
        .arg("-f")
        .arg("bin")
        .arg("-o")
        .arg(format!("{out_dir}/trampoline"))
        .arg("src/asm/trampoline.asm")
        .status()
        .expect("failed to run nasm");

    assert!(status.success(), "nasm failed with exit status {status}");

    println!("cargo:rerun-if-changed=src/asm/trampoline.asm");
}
