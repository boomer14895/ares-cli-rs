use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

use ares_common_connection::session::NewSession;
use ares_common_device::DeviceManager;
use install::InstallApp;
use list::ListApps;

mod install;
mod list;

#[derive(Parser, Debug)]
#[command(about)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "DEVICE",
        env = "ARES_DEVICE",
        help = "Specify DEVICE to use"
    )]
    device: Option<String>,
    #[arg(short, long, group = "action", help = "List the installed apps")]
    list: bool,
    #[arg(
        short,
        long,
        group = "action",
        value_name = "APP_ID",
        help = "Remove app with APP_ID"
    )]
    remove: Option<String>,
    #[arg(
        value_name = "PACKAGE_FILE",
        group = "action",
        help = "webOS package with .ipk extension"
    )]
    package: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let manager = DeviceManager::default();
    let device = manager.find_or_default(cli.device).unwrap();
    if device.is_none() {
        eprintln!("Device not found");
        exit(1);
    }
    let device = device.unwrap();
    let session = device.new_session().unwrap();
    if cli.list {
        session.list_apps();
    } else if let Some(id) = cli.remove {
        println!("Removing {id}...");
    } else if let Some(package) = cli.package {
        if let Some(file_name) = package.file_name() {
            println!(
                "Installing {} on {}...",
                file_name.to_string_lossy(),
                device.name
            );
        } else {
            println!(
                "Installing {} on {}...",
                package.to_string_lossy(),
                device.name
            );
        }
        match session.install_app(package) {
            Ok(package_id) => println!("{package_id} installed."),
            Err(e) => {
                eprintln!("Failed to install: {e:?}");
                exit(1);
            }
        }
    } else {
        Cli::parse_from(vec!["", "--help"]);
    }
}
