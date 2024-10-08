//TODO FIX
#![allow(dead_code)]
#![allow(unused)]

use std::sync::{Arc, Mutex};

use clap::{Parser, ValueEnum};

#[cfg(feature = "xclip")]
mod clipboard_xclip;
mod execution;
mod providers;
mod ui;

const DEFAULT_OPTION: i32 = 0; // Copy to clipboard entry
const DEFAULT_UI_SCALE: f64 = 1.5;
static DBG_URL: &str = "http://example.com/this/is.a.url?all=right";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum UI {
    CLI,
    #[cfg(feature = "gtk4")]
    GTK,
    #[cfg(feature = "relm4")]
    Relm,
    #[cfg(feature = "iced")]
    Iced,
    TestProviders,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ClipboardBackend {
    #[cfg(feature = "arboard")]
    Arboard,
    #[cfg(feature = "xclip")]
    Xclip,
    #[cfg(feature = "gtk4")]
    Gtk,
    #[cfg(feature = "clipboard_extras")]
    CliClipboard,
}

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value_t = DEFAULT_OPTION)]
    #[arg(short, long = "default")]
    default_option: i32,

    #[arg(default_value_t = DEFAULT_UI_SCALE)]
    #[arg(short = 's', long)]
    ui_scale: f64,

    #[arg(value_enum)]
    #[arg(default_value_t = ClipboardBackend::Xclip)]
    #[arg(long)]
    #[arg(help = "Which clipboard backend to use for copy")]
    clipboard_backend: ClipboardBackend,

    #[arg(value_enum)]
    #[arg(default_value_t = UI::Relm)]
    #[arg(short, long)]
    #[arg(help = "Choose the ui to use")]
    ui: UI,

    // Shortcuts
    #[arg(short, long)]
    #[arg(help = "Same as --ui cli")]
    cli: bool,

    #[cfg(feature = "gtk4")]
    #[arg(short, long)]
    #[arg(help = "Same as --ui gtk")]
    gtk: bool,

    #[cfg(feature = "relm4")]
    #[arg(short, long)]
    #[arg(help = "Same as --ui relm")]
    relm: bool,

    #[cfg(feature = "iced")]
    #[arg(short, long)]
    #[arg(help = "Same as --ui iced")]
    iced: bool,

    #[arg(short, long)]
    #[arg(help = "Do not execute the selected command (dry run for debug)")]
    no_exec: bool,

    #[arg(default_value_t = DBG_URL.to_string())]
    url: String,
}

static _CLI_ARGS: Mutex<Option<Arc<Cli>>> = Mutex::new(None);

pub fn get_cli_args() -> Arc<Cli> {
    _CLI_ARGS.lock().unwrap().clone().clone().unwrap()
}

fn setup_cli_args() -> Cli {
    let mut cli = Cli::parse();

    if cli.cli {
        cli.ui = UI::CLI;
    }
    #[cfg(feature = "gtk4")]
    if cli.gtk {
        cli.ui = UI::GTK;
    }
    #[cfg(feature = "relm4")]
    if cli.relm {
        cli.ui = UI::Relm;
    }
    #[cfg(feature = "iced")]
    if cli.iced {
        cli.ui = UI::Iced;
    }

    println!("\tUI: {:?}", cli.ui);
    println!("\tURL: {}", cli.url);

    *_CLI_ARGS.lock().unwrap() = Some(Arc::new(cli.clone()));
    cli
}

fn main() {
    let cli_args = setup_cli_args();

    match cli_args.ui {
        UI::CLI => {
            return execution::choose_and_execute(
                ui::ui_cli::chooser,
                cli_args.url,
                cli_args.default_option,
            )
        }
        #[cfg(feature = "gtk4")]
        UI::GTK => {
            return execution::choose_and_execute(
                ui::ui_gtk4::chooser,
                cli_args.url,
                cli_args.default_option,
            )
        }
        #[cfg(feature = "relm4")]
        UI::Relm => {
            return execution::choose_and_execute(
                ui::ui_relm4::chooser,
                cli_args.url,
                cli_args.default_option,
            )
        }
        #[cfg(feature = "iced")]
        UI::Iced => unimplemented!(),
        UI::TestProviders => return providers::main_dev(),
    }
}
