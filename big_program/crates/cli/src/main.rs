use my_lib::Module;
use log::{info, error, debug};
use env_logger;
use structopt::{clap::AppSettings, StructOpt};
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "wasm-runner",
    about = "Sample project from https://vino.dev/blog/node-to-rust-day-1-rustup/",
    global_settings(&[
      AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {
     /// The WebAssembly file to load.
     #[structopt(parse(from_os_str))]
     pub(crate) file_path: PathBuf,
}

fn main() {
    env_logger::init();
    debug!("Initialized logger");

    let options = CliOptions::from_args();

    match Module::from_file(&options.file_path) {
        Ok(_) => {
            info!("Module loaded");
        }
        Err(e) => {
            error!("Module failed to load: {}", e);
        }
      }
}

