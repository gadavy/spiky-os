use clap::{Parser, Subcommand};
use std::process::Command;

const UEFI_PATH: &str = env!("UEFI_PATH");

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Flash {
        device: String,
    },
    Qemu {
        #[arg(long, default_value_t = 1)]
        cpu_count: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Flash { device } => flash(&device),
        Commands::Qemu { cpu_count } => qemu(cpu_count),
    }
}

fn qemu(cpu_count: u8) {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={UEFI_PATH}"))
        .arg("-smp")
        .arg(format!("cpus={cpu_count}"))
        .arg("-serial")
        .arg("stdio");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}

#[cfg(target_os = "macos")]
fn flash(device: &str) {
    use std::process::Stdio;

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
fn flash(_device: &str) {
    println!("flash command unsupported.")
}
