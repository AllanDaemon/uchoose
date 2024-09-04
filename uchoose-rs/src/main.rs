use clap::{Parser, ValueEnum};
use providers::BrowserEntry;

mod providers;
mod ui;

static DBG_URL: &str = "http://example.com/this/is.a.url?all=right";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UI {
    CLI,
    GTK,
    Iced,
    TestProviders,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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
    if cli.iced {
        cli.ui = UI::Iced;
    }

    println!("\tUI: {:?}", cli.ui);
    println!("\tURL: {}", cli.url);

    match cli.ui {
        UI::CLI => return choose_and_execute(ui::ui_cli::chooser, cli.url),
        UI::GTK => return choose_and_execute(ui::ui_gtk4::chooser, cli.url),
        UI::Iced => unimplemented!(),
        UI::TestProviders => return providers::main_dev(),
    }
}

fn choose_and_execute(chooser: ui::Chooser, url: String) {
    const DEFAULT_OPTION: ui::Choice = 0;

    // Choose
    let browser_list: Vec<BrowserEntry> = providers::get_browsers_list();

    let choice = chooser(url, &browser_list, DEFAULT_OPTION);
    let browser = browser_list[choice].clone();

    println!("CHOICE: {:?} [{:#?}]", choice, browser);

    // Execute
}
