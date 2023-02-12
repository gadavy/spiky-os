use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

const UEFI_PATH: &str = env!("UEFI_PATH");

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Flash { device: String },
    Qemu,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Flash { device } => flash(&device),
        Commands::Qemu => qemu(),
    }
}

fn qemu() {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-smp")
        .arg("cpus=4")
        .arg("-serial")
        .arg("stdio")
        .arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={UEFI_PATH}"));

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}

#[cfg(target_os = "macos")]
fn flash(device: &str) {
    if !Command::new("diskutil")
        .arg("info")
        .arg(device)
        .stdout(Stdio::null())
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    if !Command::new("diskutil")
        .arg("unmountDisk")
        .arg(device)
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    if !Command::new("dd")
        .arg(format!("if={UEFI_PATH}"))
        .arg(format!("of={device}"))
        .arg("bs=1m")
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    Command::new("diskutil")
        .arg("eject")
        .arg(device)
        .status()
        .unwrap();
}

#[cfg(not(any(target_os = "macos")))]
fn flash(device: &str) {
    println!("flash command unsupported.")
}
