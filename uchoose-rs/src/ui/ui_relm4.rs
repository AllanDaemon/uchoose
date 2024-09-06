use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::prelude::*;
use relm4::{prelude::*, RelmObjectExt};

use super::{Choice, ChoiceIndex};
use crate::providers::{BrowserEntry, EntryAction};

const APP_ID: &str = "gg.allan.uchoose.rs.relm4";
const PADDING_SIZE: i32 = 16;
const UI_SCALE: f64 = 1.5;

struct UchooseApp {
    result: Rc<RefCell<Choice>>, // The way to get write the result back to us
}

#[derive(Debug)]
enum InputMsg {
    Chosen(ChoiceIndex),
    Cancelled,
}

#[derive(Debug, Clone)]
struct UchooseParams {
    url: String,
    browser_list: Vec<BrowserEntry>,
    default_option: ChoiceIndex,
    result: Rc<RefCell<Choice>>, // Used as intermediary to build in the model
}

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("Relm4 Open: {}", url);

    let result = Rc::new(RefCell::new(None));

    let mut choose_params: UchooseParams = UchooseParams {
        url: url.to_string(),
        browser_list: browser_list.clone(),
        default_option: default_option.clone(),
        result: Rc::clone(&result),
    };

    // Pass empty args, otherwise it interpret our args as gtk args
    let gtk_app_arg: Vec<String> = Vec::new();
    let app = RelmApp::new(APP_ID).with_args(gtk_app_arg);

    println!("App will run");
    app.run::<UchooseApp>(choose_params);
    println!("App ran out\n");

    return *result.borrow();
}

struct AppWidgets {}

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
        chooser_cancel_connect(sender.clone(), window.clone());

        let model = UchooseApp {
            result: init_params.result,
        };

        let label = gtk::Label::new(Some(&init_params.url));
        label.set_margin_all(PADDING_SIZE);
        label.set_selectable(true); // Set windows focus latter to avoid starting selected

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(PADDING_SIZE)
            .build();
        vbox.set_margin_all(5);
        vbox.append(&label);
        window.set_child(Some(&vbox));

        let icon_theme = gtk::IconTheme::default();

        for (idx_btn, entry) in init_params.browser_list.iter().enumerate() {
            let label = gtk::Label::new(Some(&entry.name));

            let icon = gtk::Image::builder()
                .icon_name(entry.icon.to_owned())
                .icon_size(gtk::IconSize::Large)
                .build();

            let btn_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(PADDING_SIZE)
                .build();
            btn_box.append(&icon);
            btn_box.append(&label);

            let btn_tooltip_text = match &entry.action {
                EntryAction::Exec(s) => s,
                _ => &String::new(),
            };
            let is_default: bool = idx_btn == init_params.default_option;

            let btn = gtk::Button::builder()
                .child(&btn_box)
                .tooltip_text(btn_tooltip_text)
                .receives_default(is_default)
                .build();

            vbox.append(&btn);

            if idx_btn == init_params.default_option {
                gtk::prelude::GtkWindowExt::set_focus(&window, Some(&btn));
            }

            btn.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(InputMsg::Chosen(idx_btn));
                }
            ));
        }

        let widgets = AppWidgets {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let choice = match msg {
            InputMsg::Chosen(choice) => {
                println!("InputMsg::Chosen {choice:?}");
                Some(choice)
            }
            InputMsg::Cancelled => {
                println!("InputMsg::Cancelled");
                None
            }
        };

        // All input messages share this set and quit code;
        *self.result.borrow_mut() = choice;
        relm4::main_application().quit();
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

fn chooser_cancel_connect(sender: ComponentSender<UchooseApp>, window: gtk::Window) {
    // Abort when press esc
    let event_controler = gtk::EventControllerKey::new();
    event_controler.connect_key_pressed(clone!(
        #[strong]
        sender,
        move |this: &gtk::EventControllerKey,
              keyval: gtk::gdk::Key,
              keycode: u32,
              state: gtk::gdk::ModifierType| {
            if keyval == gtk::gdk::Key::Escape {
                println!("ESC");
                sender.input(InputMsg::Cancelled);
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        },
    ));
    window.add_controller(event_controler);

    // Abort when close the window
    window.connect_close_request(clone!(
        #[strong]
        sender,
        move |_| {
            sender.input(InputMsg::Cancelled);
            gtk::glib::Propagation::Stop
        }
    ));
}
