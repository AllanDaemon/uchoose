use clap::{Parser, ValueEnum};

static DBG_URL: &str = "http://example.com/this/is.a.url?all=right";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UI {
    CLI,
    GTK,
    Iced,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    #[arg(default_value_t = UI::CLI)]
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
}
