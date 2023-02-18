use clap::{Args, Parser, Subcommand};
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
    /// Flash an kernel to a target device
    Flash(FlashArgs),
    /// Run kernel in QEMU
    Qemu(QemuArgs),
}

#[derive(Debug, Args)]
struct FlashArgs {
    /// Target device
    device: String,
}

#[derive(Debug, Args)]
struct QemuArgs {
    #[arg(long, default_value_t = 1)]
    cpu_count: u8,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Flash(args) => flash(&args),
        Commands::Qemu(args) => qemu(&args),
    }
}

fn qemu(args: &QemuArgs) {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={UEFI_PATH}"))
        .arg("-smp")
        .arg(format!("cpus={}", args.cpu_count))
        .arg("-serial")
        .arg("stdio");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}

#[cfg(target_os = "macos")]
fn flash(args: &FlashArgs) {
    use std::process::Stdio;

    if !Command::new("diskutil")
        .arg("info")
        .arg(&args.device)
        .stdout(Stdio::null())
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    if !Command::new("diskutil")
        .arg("unmountDisk")
        .arg(&args.device)
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    if !Command::new("dd")
        .arg(format!("if={UEFI_PATH}"))
        .arg(format!("of={}", args.device))
        .arg("bs=1m")
        .status()
        .unwrap()
        .success()
    {
        return;
    }

    Command::new("diskutil")
        .arg("eject")
        .arg(&args.device)
        .status()
        .unwrap();
}

#[cfg(not(any(target_os = "macos")))]
fn flash(_args: &FlashArgs) {
    println!("flash command unsupported.")
}
