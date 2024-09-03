use std::borrow::Cow;
use std::path::PathBuf;
use std::{fs, path::Path};

use ini::Ini;

const APP_DIR_SYSTEM: &str = "/usr/share/applications/";
const APP_DIR_USER: &str = "~/.local/share/applications/";
const DESKTOP_EXTENSION: &str = "desktop"; // Files that ends with .desktop
const WEB_BROWSER_CATEGORY: &str = "WebBrowser";
const USE_USER_APPS: bool = false;

#[derive(Debug)]
pub struct BrowserEntry {
    name: Cow<'static, str>,
    icon: Cow<'static, str>,
    exec: Option<Cow<'static, str>>,
}

pub const CLIPBOARD_ENTRY: BrowserEntry = BrowserEntry {
    name: Cow::Borrowed("Copy to clipboard"),
    icon: Cow::Borrowed("edit-copy-symbolic"),
    exec: None,
};

fn get_app_desktop_paths(app_dir: &Path) -> Vec<PathBuf> {
    assert!(app_dir.is_dir());

    let mut desktop_entries: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(app_dir).unwrap() {
        let entry: fs::DirEntry = entry_result.unwrap();
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == DESKTOP_EXTENSION {
                desktop_entries.push(path);
            }
        }
    }

    desktop_entries
}

fn get_app_browser_ini(path: &PathBuf) -> Option<BrowserEntry> {
    // println!("FILE: {:?}", path);
    let ini = Ini::load_from_file(path).expect("Error parsing desktop file");

    let desktop_entry = ini
        .section(Some("Desktop Entry"))
        .expect("No desktop.entry")
        .clone();
    // println!("{:#?}", desktop_entry);

    let _categories = desktop_entry.get("Categories").unwrap_or_default();
    let categories: Vec<&str> = _categories.split(';').collect();

    if !categories.contains(&WEB_BROWSER_CATEGORY) {
        return None; // Not a Web Browser
    }

    let name = Cow::Owned(desktop_entry.get("Name").unwrap().into());
    let icon = Cow::Owned(desktop_entry.get("Icon").unwrap().into());
    let exec = Cow::Owned(desktop_entry.get("Exec").unwrap().into());

    let entry: BrowserEntry = BrowserEntry {
        name: name,
        icon: icon,
        exec: Some(exec),
    };

    return Some(entry);
}

fn get_browser_desktop_list() -> Vec<BrowserEntry> {
    let app_dir = Path::new(APP_DIR_SYSTEM);
    let mut file_paths = get_app_desktop_paths(&app_dir);

    if USE_USER_APPS {
        let app_dir_user = Path::new(APP_DIR_USER);
        let file_paths_user = get_app_desktop_paths(&app_dir_user);
        file_paths.extend(file_paths_user);
    }

    let browsers: Vec<BrowserEntry> = file_paths
        .iter()
        .map(get_app_browser_ini)
        .flatten()
        .collect();

    browsers
}

pub fn get_browsers_list() -> Vec<BrowserEntry> {
    let mut browsers: Vec<BrowserEntry> = get_browser_desktop_list();
    browsers.insert(0, CLIPBOARD_ENTRY);

    browsers
}

pub fn main_dev() {
    let browsers = get_browsers_list();
    println!("BROWSERS:\n{:#?}", browsers);
}
