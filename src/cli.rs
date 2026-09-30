use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug)]
pub enum VCommand {
    Build { target: String },
    Publish { crate_name: String, version: String },
    Resolve { domain: String },
    Unknown(String),
}

pub struct VCli;

impl VCli {
    /// Menganalisis hujah baris arahan (CLI args)
    pub fn parse_args(args: &[String]) -> VCommand {
        if args.len() < 2 {
            return VCommand::Unknown("Mesej: Sila masukkan arahan (cth: v build, v publish, v resolve)".to_string());
        }

        match args[1].as_str() {
            "build" => {
                let target = args.get(2).cloned().unwrap_or_else(|| "main.volt".to_string());
                VCommand::Build { target }
            }
            "publish" => {
                let crate_name = args.get(2).cloned().unwrap_or_else(|| "unnamed_crate".to_string());
                let version = args.get(3).cloned().unwrap_or_else(|| "0.1.0".to_string());
                VCommand::Publish { crate_name, version }
            }
            "resolve" => {
                let domain = args.get(2).cloned().unwrap_or_else(|| "shadow-rpg.volt".to_string());
                VCommand::Resolve { domain }
            }
            cmd => VCommand::Unknown(cmd.to_string()),
        }
    }

    /// Menjalankan logik utama arahan CLI v
    pub fn execute(cmd: VCommand) -> String {
        match cmd {
            VCommand::Build { target } => {
                let mut out = String::from("[Volt CLI] Membina fail: ");
                out.push_str(&target);
                out.push_str("\n[Volt CLI] Status: Success -> Output: ");
                out.push_str(&target.replace(".volt", ".vbin"));
                out
            }
            VCommand::Publish { crate_name, version } => {
                let mut out = String::from("[Volt CLI] Menerbitkan ");
                out.push_str(&crate_name);
                out.push_str(" v");
                out.push_str(&version);
                out.push_str(" ke SQL Mirror tempatan...");
                out
            }
            VCommand::Resolve { domain } => {
                let mut out = String::from("[Volt Resolver] Menyerahkan domain ");
                out.push_str(&domain);
                out.push_str(" -> IP Direct: 192.168.1.100 (Bypassed ICANN)");
                out
            }
            VCommand::Unknown(cmd) => {
                let mut out = String::from("[Volt CLI] Arahan tidak dikenali: ");
                out.push_str(&cmd);
                out
            }
        }
    }
}
