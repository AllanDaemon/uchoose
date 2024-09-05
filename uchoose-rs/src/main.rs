//TODO FIX
#![allow(dead_code)]
#![allow(unused)]

use clap::{Parser, ValueEnum};
use providers::BrowserEntry;

mod providers;
mod ui;

const DEFAULT_OPTION: i32 = 0;
static DBG_URL: &str = "http://example.com/this/is.a.url?all=right";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UI {
    CLI,
    GTK,
    Relm,
    Iced,
    TestProviders,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = DEFAULT_OPTION)]
    #[arg(short, long)]
    default: i32,

    #[arg(value_enum)]
    #[arg(default_value_t = UI::GTK)]
    #[arg(short, long)]
    #[arg(help = "Choose the ui to use")]
    ui: UI,

    // Shortcuts
    #[arg(short, long)]
    #[arg(help = "Same as --ui cli")]
    cli: bool,

    #[arg(short, long)]
    #[arg(help = "Same as --ui gtk")]
    gtk: bool,

    #[arg(short, long)]
    #[arg(help = "Same as --ui relm")]
    relm: bool,

    #[arg(short, long)]
    #[arg(help = "Same as --ui iced")]
    iced: bool,

    #[arg(default_value_t = DBG_URL.to_string())]
    url: String,
}

fn main() {
    let mut cli = Cli::parse();

    if cli.cli {
        cli.ui = UI::CLI;
    }
    if cli.gtk {
        cli.ui = UI::GTK;
    }
    if cli.relm {
        cli.ui = UI::Relm;
    }
    if cli.iced {
        cli.ui = UI::Iced;
    }

    println!("\tUI: {:?}", cli.ui);
    println!("\tURL: {}", cli.url);

    match cli.ui {
        UI::CLI => return choose_and_execute(ui::ui_cli::chooser, cli.url, cli.default),
        UI::GTK => return choose_and_execute(ui::ui_gtk4::chooser, cli.url, cli.default),
        UI::Relm => return choose_and_execute(ui::ui_relm4::chooser, cli.url, cli.default),
        UI::Iced => unimplemented!(),
        UI::TestProviders => return providers::main_dev(),
    }
}

// default: 0 is the 1st in the list; 1 is the 2nd; -1 is the last; -2 is the second to last;
fn choose_and_execute(chooser: ui::Chooser, url: String, default: i32) {
    // Choose
    let browser_list: Vec<BrowserEntry> = providers::get_browsers_list();

    let _default: ui::Choice = if default < 0 {
        (browser_list.len() as i32 + default).max(0)
    } else {
        default
    } as ui::Choice;

    let choice = chooser(url, &browser_list, _default);
    let browser = browser_list[choice].clone();

    println!("CHOICE: {:?} [{:#?}]", choice, browser);

    // Execute
}
