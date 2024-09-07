// Needs to install libraries in the system too
// Check: https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html

use std::borrow::Borrow;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

const APP_ID: &str = "gg.allan.uchoose.rs.gkt4";

const MARGIN: i32 = 16;

#[derive(Debug)]
struct ChoiceResult(Option<Choice>);

#[derive(Debug, Clone)]
struct UchooseWin {
    url: String,
    browser_list: Vec<BrowserEntry>,
    default_option: ChoiceIndex,
    choice: Option<Choice>,
    state: Option<UchooseWinState>,
}

#[derive(Debug, Clone)]
struct UchooseWinState {
    win: ApplicationWindow,
}

impl UchooseWin {
    fn choose(&mut self) -> Choice {
        println!("App create");
        let app = Application::builder().application_id(APP_ID).build();

        // println!("App activate");
        // app.connect_activate(|app| self.build_uchoose(app));

        println!("App build");
        self.build_uchoose(&app);

        println!("App run");
        app.run();
        println!("App run out");

        println!("CHOICE: {:#?}", self.choice);

        None
    }

    fn build_uchoose(&mut self, app: &Application) {
        // let icon_theme;
        let vbox = gtk::Box::builder()
            // .orientation(gtk::Orientation::Vertical)
            // .margin_start(2 * MARGIN)
            // .margin_end(2 * MARGIN)
            // .margin_top(MARGIN)
            // .margin_bottom(MARGIN)
            .build();

        let window: ApplicationWindow = ApplicationWindow::builder()
            .application(app)
            .title("uchoose")
            .child(&vbox)
            .build();

        let win = &window;
        // self.state = Some(UchooseWinState { win: window });

        // let url_label = gtk::Label::new(Some(url));
        let url_label = gtk::Label::builder()
            .label(&self.url)
            .margin_bottom(MARGIN)
            .build();
        vbox.append(&url_label);

        for (i, entry) in self.browser_list.iter().enumerate() {
            let label = gtk::Label::new(Some(entry.name.borrow()));

            let btn = gtk::Button::builder()
                .icon_name(entry.icon.to_owned())
                .label(entry.name.to_owned())
                .has_frame(true)
                .margin_top(8)
                .margin_bottom(8)
                .build();

            btn.connect_clicked(move |_btn: &gtk::Button| {
                println!("<BUTTON PRESSED> {}", i);

                // win.destroy();
            });

            vbox.append(&btn);
        }

        win.present();
    }

    fn on_click(&self, choice: Choice) {}
}

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("GTK4 Open: {}", url);

    let mut chooser: UchooseWin = UchooseWin {
        url: url.to_string(),
        browser_list: browser_list.clone(),
        default_option: default_option.clone(),
        choice: None,
        state: None,
    };

    chooser.choose()
}

/*

fn build_uchoose(
    app: &Application,
    url: &str,
    browser_list: &Vec<BrowserEntry>,
    default: Choice,
    // result: &mut ChoiceResult,
) {
    // let icon_theme;

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(2 * MARGIN)
        .margin_end(2 * MARGIN)
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("uchoose")
        .child(&vbox)
        .build();

    // let url_label = gtk::Label::new(Some(url));
    let url_label = gtk::Label::builder()
        .label(url)
        .margin_bottom(MARGIN)
        .build();
    vbox.append(&url_label);

    for (i, entry) in browser_list.iter().enumerate() {
        let label = gtk::Label::new(Some(entry.name.borrow()));

        let btn = gtk::Button::builder()
            .icon_name(entry.icon.to_owned())
            .label(entry.name.to_owned())
            .has_frame(true)
            .margin_top(8)
            .margin_bottom(8)
            .build();

        btn.connect_clicked(move |_btn| {
            println!("<BUTTON PRESSED> {}", i);
            window.destroy();
        });

        vbox.append(&btn);
    }

    window.present();
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

pub fn chooser2(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("GTK4 Open: {}", url);

    // let mut choice = ChoiceResult(None);

    let app = Application::builder().application_id(APP_ID).build();
    let _browser_list = browser_list.clone();
    app.connect_activate(move |app| build_uchoose(app, &url, &_browser_list, default));

    println!("App run");
    app.run();
    println!("App run out");
    // println!("CHOICE: {:#?}", choice);

    0
} */
