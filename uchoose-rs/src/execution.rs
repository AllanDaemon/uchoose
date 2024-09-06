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

fn execute_clipboad(url: &str, clipboard_backend: ClipboardBackend) {
    match clipboard_backend {
        ClipboardBackend::Arboard => {
            let mut clipboard = arboard::Clipboard::new().unwrap();
            clipboard.set_text(url).unwrap();
            println!("URL copied to clipboard: {url}");
        }
        ClipboardBackend::Xclip => {
            // try xclip -selection clipboard
        } // use gtk::gdk::Clipboard;
    }
}

fn execute_exec(url: &str, exec: &str) {}
