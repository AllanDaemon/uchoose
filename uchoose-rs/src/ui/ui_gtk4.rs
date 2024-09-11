// Needs to install libraries in the system too
// Check: https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html

use std::cell::RefCell;
use std::rc::Rc;

// use std::borrow::Borrow;

use gtk::glib::clone;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Window};

use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

const APP_ID: &str = "gg.allan.uchoose.rs.gkt4";

const MARGIN: i32 = 16;

#[derive(Debug)]
struct ChoiceResult(Option<Choice>);

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("GTK4 Open: {}", url);

    let result = Rc::new(RefCell::new(None));

    let app = Application::builder().application_id(APP_ID).build();
    let _browser_list = browser_list.clone();

    let url_clone = url.to_owned().clone();

    let result_clone = Rc::clone(&result);
    // app.connect_activate(|app| {
    //     let vbox = gtk::Box::builder()
    //         .orientation(gtk::Orientation::Vertical)
    //         .margin_start(2 * MARGIN)
    //         .margin_end(2 * MARGIN)
    //         .margin_top(MARGIN)
    //         .margin_bottom(MARGIN)
    //         .build();
    // });
    app.connect_activate(clone!(
        #[strong]
        result,
        #[strong]
        result_clone,
        move |app| {
            build_uchoose(
                app,
                &url_clone,
                &_browser_list.clone(),
                default_option,
                result_clone,
            )
        }
    ));

    // build_uchoose(
    //     &app,
    //     &url_clone,
    //     &_browser_list.clone(),
    //     default_option,
    //     result,
    // );

    // dbg!(&result);

    println!("App run");
    app.run_with_args(&[] as &[&str]);
    println!("App run out");

    // let _choice: RefCell<Option<ChoiceIndex>> = result;
    // let choice: Option<ChoiceIndex> = *result.borrow();
    // dbg!(&result);
    // println!("CHOICE: {:#?}", result);

    None
}

fn build_uchoose(
    app: &Application,
    url: &str,
    browser_list: &Vec<BrowserEntry>,
    default: ChoiceIndex,
    result: Rc<RefCell<Choice>>,
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
        let label = gtk::Label::new(Some(std::borrow::Borrow::borrow(&entry.name)));

        let btn = gtk::Button::builder()
            .icon_name(entry.icon.to_owned())
            .label(entry.name.to_owned())
            .has_frame(true)
            .margin_top(8)
            .margin_bottom(8)
            .build();

        // let win = &window;
        // let res = Rc::clone(&result);
        btn.connect_clicked(clone!(
            #[strong]
            window,
            #[strong]
            result,
            move |_btn| {
                println!("<BUTTON PRESSED> {}", i);
                *result.borrow_mut() = Some(i as ChoiceIndex);
                window.destroy();
            }
        ));

        vbox.append(&btn);
    }

    window.present();
}
