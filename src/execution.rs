use gtk::prelude::DisplayExt; // For GTK clipboard

use crate::providers::{get_browsers_list, BrowserEntry, EntryAction};
use crate::ui;
use crate::{get_cli_args, ClipboardBackend};

// default: 0 is the 1st in the list; 1 is the 2nd; -1 is the last; -2 is the second to last;
pub fn choose_and_execute(chooser: ui::Chooser, url: String, default_option: i32) {
    // Choose
    let browser_list: Vec<BrowserEntry> = get_browsers_list();

    let default_option: ui::ChoiceIndex = if default_option < 0 {
        (browser_list.len() as i32 + default_option).max(0)
    } else {
        default_option
    } as ui::ChoiceIndex;

    let choice = chooser(&url, &browser_list, default_option);
    if let None = choice {
        println!("Choosing cancelled. No action will be taken.");
        return;
    }

    let entry = browser_list[choice.unwrap()].clone();

    println!("CHOICE: {:?} [{:#?}]", choice, entry);

    // Execute
    execute(&url, &entry)
}

fn execute(url: &str, entry: &BrowserEntry) {
    println!("Executing {entry:?}");

    match &entry.action {
        EntryAction::None => (),
        EntryAction::Clipboard => execute_clipboad(url, get_cli_args().clipboard_backend),
        EntryAction::Exec(exec) => execute_exec(url, exec),
    }
}

/// CLIPBOARD PART ///

fn execute_clipboad(url: &str, clipboard_backend: ClipboardBackend) {
    match clipboard_backend {
        ClipboardBackend::Xclip => {
            crate::clipboard_xclip::clipboard_set_text(url);
        }
        ClipboardBackend::Arboard => {
            let mut clipboard = arboard::Clipboard::new().unwrap();
            clipboard.set_text(url).unwrap();
            println!("URL copied to clipboard: {url}");
        }
        ClipboardBackend::Gtk => {
            let display = gtk::gdk::Display::default().unwrap();
            let clipboard = display.clipboard();
            clipboard.set_text(url);
        }
        ClipboardBackend::CliClipboard => {
            cli_clipboard::set_contents(url.to_owned()).unwrap();
        }
    }
}

/// EXEC PART ///

// https://developer.gnome.org/integration-guide/stable/desktop-files.html.en#tb-exec-params
pub fn param_substitution_simplify(arg: String) -> String {
    arg.replace("%U", "%u").replace("%k", "%u")
}

fn params_prepare(url: &str, exec: &str) -> (String, Vec<String>) {
    // split the exec string
    let cmd_parts: Vec<String> = shlex::split(&exec).unwrap();
    let mut converted: Vec<String> = cmd_parts
        .into_iter()
        .map(param_substitution_simplify)
        .collect();

    // If url parameter isn't in the arguments, add it
    // TODO: Handle if %u isn't a whole parameter
    if !converted.contains(&String::from("%u")) {
        converted.push("%u".into());
    }

    // Convert all %u to the url
    let mut final_args: Vec<String> = converted
        .into_iter()
        .map(|arg| arg.replace("%u", url))
        .collect();

    let argv0 = final_args.remove(0);
    (argv0, final_args)
}

fn execute_exec(url: &str, exec: &str) {
    let (cmd, params) = params_prepare(url, exec);
    println!("Exec {cmd} {params:#?}");
    dbg!(&cmd);
    dbg!(&params);

    let res = std::process::Command::new(cmd).args(&params).spawn();

    if let Err(err) = res {
        eprintln!("Error launching the selected program");
        eprintln!("{:#?}", err);
    } else {
        println!("Launched successfully")
    }
}