fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-smp")
        .arg("cpus=4")
        .arg("-serial")
        .arg("stdio")
        .arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={uefi_path}"));

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
