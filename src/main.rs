mod config;
mod gestures;
mod ipc;
mod ipc_client;
mod utils;

#[cfg(test)]
mod tests;

use std::{path::PathBuf, rc::Rc, thread};

use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;
use miette::Result;

use crate::config::*;

fn main() -> Result<()> {
    let app = App::parse();

    {
        let mut l = Builder::from_default_env();

        if app.verbose > 0 {
            l.filter_level(match app.verbose {
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                _ => LevelFilter::max(),
            });
        }

        if app.debug {
            l.filter_level(LevelFilter::Debug);
        }

        l.init();
    }

    let c = if let Some(p) = app.conf {
        Config::read_from_file(&p)?
    } else {
        config::Config::read_default_config().unwrap_or_else(|_| {
            log::error!("Could not read configuration file, using empty config!");
            Config::default()
        })
    };
    log::debug!("{:#?}", &c);

    match app.command {
        Commands::Reload => {}
        Commands::Start => run_eh(Rc::new(c))?,
    }

    Ok(())
}

fn run_eh(config: Rc<Config>) -> Result<()> {
    let ipc_listener = thread::spawn(|| {
        ipc::create_socket();
    });
    let mut eh = gestures::EventHandler::new(config);
    let mut interface = input::Libinput::new_with_udev(gestures::Interface);
    eh.init(&mut interface)?;
    eh.main_loop(&mut interface);
    ipc_listener.join().unwrap();
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct App {
    /// Verbosity, can be repeated
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Debug mode
    #[arg(short, long)]
    debug: bool,
    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    conf: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Reload the configuration
    Reload,
    /// Start the program
    Start,
}
