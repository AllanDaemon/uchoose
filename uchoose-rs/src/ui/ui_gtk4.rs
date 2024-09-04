// Needs to install libraries in the system too
// Check: https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use super::Choice;
use crate::providers::BrowserEntry;

const APP_ID: &str = "gg.allan.uchoose.rs.gkt4";

fn build_uchoose(app: &Application, url: &str, browser_list: &Vec<BrowserEntry>, default: Choice) {
    build_ui2(app)
}

fn build_ui2(app: &Application) {
    // Create a button with label and margins
    let button = gtk::Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}

pub fn chooser(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("GTK4 Open: {}", url);

    let app = Application::builder().application_id(APP_ID).build();

	let _browser_list = browser_list.clone();

    app.connect_activate(move |app| build_uchoose(app, &url, &_browser_list, default));

    println!("App run");
    app.run();
    println!("App run out");

    0
}
