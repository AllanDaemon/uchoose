use gtk::glib::clone;
use gtk::prelude::*;
use relm4::prelude::*;

use super::Choice;
use crate::providers::BrowserEntry;

const APP_ID: &str = "gg.allan.uchoose.rs.relm4";
const MARGIN: i32 = 16;
const UI_SCALE: f64 = 1.5;

struct UchooseApp {
    choice: Option<Choice>,
}

#[derive(Debug)]
enum InputMsg {
    Chosen,
    Cancelled,
}

#[derive(Debug, Clone)]
struct UchooseParams {
    url: String,
    browser_list: Vec<BrowserEntry>,
    default: Choice,
}

pub fn chooser(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("Relm4 Open: {}", url);

    let choose_params: UchooseParams = UchooseParams {
        url: url.clone(),
        browser_list: browser_list.clone(),
        default: default.clone(),
    };

    // Pass empty args, otherwise it interpret our args as gtk args
    let gtk_app_arg: Vec<String> = Vec::new();
    let app = RelmApp::new(APP_ID).with_args(gtk_app_arg);

    println!("App will run");
    app.run::<UchooseApp>(choose_params);
    println!("App ran out");

    0
}

struct AppWidgets {
    // window: gtk::Window,
    // vbox: gtk::Box,
    // inc_button: gtk::Button,
    // dec_button: gtk::Button,
    label: gtk::Label,
}

fn set_scale(win: &gtk::Window, scale: f64) {
    // Get settings and set scale
    let settings = win.settings();

    let curr_dpi: i32 = settings.gtk_xft_dpi();
    let new_dpi: i32 = (scale * (curr_dpi as f64)) as i32;
    settings.set_gtk_xft_dpi(new_dpi);

    // println!("gtk-xft-dpi: {:#?} ({}*1024)", curr_dpi, curr_dpi / 1024);
}

impl SimpleComponent for UchooseApp {
    type Init = UchooseParams;
    type Input = InputMsg;
    type Output = ();
    type Widgets = AppWidgets;
    type Root = gtk::Window;

    fn init_root() -> Self::Root {
        let win = gtk::Window::builder().title("uChoose").build();
        set_scale(&win, UI_SCALE);
        win
    }

    fn init(
        init_params: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = UchooseApp { choice: None };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&init_params.url));
        label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&label);
        vbox.append(&inc_button);
        vbox.append(&dec_button);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(InputMsg::Chosen);
            }
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(InputMsg::Cancelled);
            }
        ));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            InputMsg::Chosen => {
                println!("InputMsg::Chosen");
                self.choice = Some(3);
            }
            InputMsg::Cancelled => {
                println!("InputMsg::Cancelled");
                self.choice = None;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
