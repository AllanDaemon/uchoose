// Needs to install libraries in the system too
// Check: https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html

use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use super::{Choice, ChoiceIndex};
use crate::get_cli_args;
use crate::providers::{BrowserEntry, EntryAction};

const APP_ID: &str = "gg.allan.uchoose.rs.gkt4";
const PADDING_SIZE: i32 = 12;

// Where to save the user choice. It's already is setup to default none (no action)
static mut RESULT: Choice = None;

fn get_result() -> Choice {
    unsafe { RESULT }
}

fn set_result(choice: Choice) {
    unsafe { RESULT = choice };
}

#[derive(Debug)]
struct ChoiceResult(Option<Choice>);

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("GTK4 Open: {}", url);

    let app = Application::builder().application_id(APP_ID).build();

    let _url = url.to_owned().clone();
    app.connect_activate(clone!(
        #[strong]
        browser_list,
        move |app| { build_uchoose(app, &_url, &browser_list, default_option) }
    ));

    println!("App run");
    app.run_with_args(&[] as &[&str]);
    println!("App run out");

    let result = get_result();
    println!("CHOICE: {:#?}", result);

    result
}

fn build_uchoose(
    app: &Application,
    url: &str,
    browser_list: &Vec<BrowserEntry>,
    default_option: ChoiceIndex,
) {
    let ui_scale = get_cli_args().ui_scale;
    let padding_size: i32 = (PADDING_SIZE as f64 * ui_scale) as i32;

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("uchoose")
        .build();
    window_exit_on_esc(&window);
    set_scale(&window, get_cli_args().ui_scale);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(padding_size)
        .margin_top(padding_size)
        .margin_bottom(padding_size)
        .margin_start(padding_size)
        .margin_end(padding_size)
        .build();

    let url_label = gtk::Label::builder()
        .label(url)
        .selectable(true) // Set windows focus latter to avoid starting selected
        .margin_top(padding_size)
        .margin_bottom(padding_size)
        .margin_start(padding_size)
        .margin_end(padding_size)
        .build();
    vbox.append(&url_label);

    let icon_theme = gtk::IconTheme::default();

    for (idx_btn, entry) in browser_list.iter().enumerate() {
        let label = gtk::Label::new(Some(&entry.name));

        let icon = gtk::Image::builder()
            .icon_name(entry.icon.to_owned())
            .icon_size(gtk::IconSize::Large)
            .build();

        let btn_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(padding_size)
            .build();
        btn_box.append(&icon);
        btn_box.append(&label);

        let btn_tooltip_text = match &entry.action {
            EntryAction::Exec(s) => s,
            _ => &String::new(),
        };
        let is_default: bool = idx_btn == default_option;

        let btn = gtk::Button::builder()
            .child(&btn_box)
            .tooltip_text(btn_tooltip_text)
            .receives_default(is_default)
            .build();

        vbox.append(&btn);

        // Select the default option to remove the selection from url_label
        if idx_btn == default_option {
            gtk::prelude::RootExt::set_focus(&window, Some(&btn));
        }

        btn.connect_clicked(clone!(
            #[strong]
            window,
            move |_btn| {
                println!("<BUTTON PRESSED> {}", idx_btn);
                set_result(Some(idx_btn as ChoiceIndex));
                window.destroy();
            }
        ));
    }

    window.set_child(Some(&vbox));
    window.present();
}

fn window_exit_on_esc(window: &gtk::ApplicationWindow) {
    // Close when press esc
    let event_controler = gtk::EventControllerKey::new();
    event_controler.connect_key_pressed(clone!(
        #[strong]
        window,
        move |this: &gtk::EventControllerKey,
              keyval: gtk::gdk::Key,
              keycode: u32,
              state: gtk::gdk::ModifierType| {
            if keyval == gtk::gdk::Key::Escape {
                println!("ESC");
                window.destroy();
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        },
    ));
    window.add_controller(event_controler);
}

pub fn set_scale(win: &gtk::ApplicationWindow, scale: f64) {
    // Get settings and set scale
    let settings = win.settings();

    let curr_dpi: i32 = settings.gtk_xft_dpi();
    let new_dpi: i32 = (scale * (curr_dpi as f64)) as i32;
    settings.set_gtk_xft_dpi(new_dpi);

    // println!("gtk-xft-dpi: {:#?} ({}*1024)", curr_dpi, curr_dpi / 1024);
}
