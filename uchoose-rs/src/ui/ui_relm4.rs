use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{prelude::*, Widget};
use relm4::factory::FactoryVecDeque;
use relm4::{prelude::*, RelmIterChildrenExt};

use super::{Choice, ChoiceIndex};
use crate::get_cli_args;
use crate::providers::{BrowserEntry, EntryAction};
use crate::ui::ui_gtk4::get_padding_size;

const APP_ID: &str = "gg.allan.uchoose.rs.relm4";

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("Relm4 Open: {}", url);

    let result = Rc::new(RefCell::new(None));

    let choose_params = UchooseParams {
        url: url.to_string(),
        browser_list: browser_list.clone(),
        default_option: default_option.clone(),
        result: Rc::clone(&result),
    };

    // Pass empty args, otherwise it interpret our args as gtk args
    let gtk_app_args: Vec<String> = Vec::new();
    let app = RelmApp::new(APP_ID).with_args(gtk_app_args);

    println!("App will run");
    app.run::<UchooseApp>(choose_params);
    println!("App ran out\n");

    return *result.borrow();
}

/*                */
// Button Factory //

#[derive(Debug, Clone)]
struct ChoiceButton {
    index: ChoiceIndex,
    name: String,
    icon_name: String,
    exec: Option<String>,
}

#[derive(Debug)]
enum ChoiceButtonOutput {
    Choice(ChoiceIndex),
}

#[relm4::factory]
impl FactoryComponent for ChoiceButton {
    type Init = ChoiceButton;
    type Input = ();
    type Output = ChoiceButtonOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        #[name="btn_wrapper_box"]
        gtk::Box{
            #[name="btn"]
            gtk::Button {
                set_tooltip_text: self.exec.as_deref(),
                // This's the default in pure GTK builder, not here in macro
                set_hexpand: true,

                connect_clicked[sender, idx=self.index.clone()] => move |_| {
                    sender.output(ChoiceButtonOutput::Choice(idx))
                        .expect("Error sending ChoiceButtonOutput::Choice()");
                },

                #[name="btn_inner_box"]
                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: get_padding_size(),

                    #[name="btn_icon"]
                    gtk::Image {
                        set_icon_name: Some(&self.icon_name),
                        set_icon_size: gtk::IconSize::Large,
                    },

                    #[name="btn_label"]
                    gtk::Label{
                        set_label: &self.name.to_string(),
                    },
                },
            }
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        value.clone()
    }
}

// Main Window //

#[derive(Debug)]
struct UchooseApp {
    url: String,
    result: Rc<RefCell<Choice>>, // Used as intermediary to build in the model
    choice_buttons: FactoryVecDeque<ChoiceButton>,
}

#[derive(Debug, Clone)]
struct UchooseParams {
    url: String,
    browser_list: Vec<BrowserEntry>,
    default_option: ChoiceIndex,
    result: Rc<RefCell<Choice>>, // Used as intermediary to build in the model
}

#[derive(Debug)]
enum InputMsg {
    Chosen(ChoiceIndex),
    Cancelled,
}

#[relm4::component]
impl SimpleComponent for UchooseApp {
    type Init = UchooseParams;
    type Input = InputMsg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("uChoose"),

            #[name="box_win"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: padding_size,
                set_margin_all: padding_size,

                #[name="url_label"]
                gtk::Label {
                    set_label: &model.url,
                    set_margin_all: padding_size,
                    set_selectable: true, // Set windows focus latter to avoid starting selected
                },

                #[local_ref]
                choice_buttons_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: padding_size,
                }
            }
        }
    }

    fn init(
        init_params: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        chooser_connect_cancel(sender.clone(), window.clone());
        super::ui_gtk4::set_scale(&window, get_cli_args().ui_scale);

        // Choice buttons factory
        let mut choice_buttons: FactoryVecDeque<ChoiceButton> = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                ChoiceButtonOutput::Choice(index) => {
                    println!("FOWARD EVENT: {output:#?}");
                    InputMsg::Chosen(index)
                }
            });

        // populate choice buttons
        {
            let mut choice_buttons_guard = choice_buttons.guard();

            for (idx_btn, entry) in init_params.browser_list.iter().enumerate() {
                choice_buttons_guard.push_back(ChoiceButton {
                    index: idx_btn,
                    name: entry.name.to_owned().to_string(),
                    icon_name: entry.icon.to_owned().to_string(),
                    exec: match entry.action.to_owned() {
                        EntryAction::Exec(cmd) => Some(cmd),
                        _ => None,
                    },
                });
            }
        }

        let model = UchooseApp {
            url: init_params.url,
            result: init_params.result,
            choice_buttons: choice_buttons,
        };

        let choice_buttons_box = model.choice_buttons.widget();

        let padding_size = get_padding_size();

        let widgets = view_output!();

        // focus default btn
        chooser_set_focus_on_button(&window, &choice_buttons_box, init_params.default_option);

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
}

fn chooser_set_focus_on_button(
    window: &gtk::ApplicationWindow,
    choice_buttons_box: &gtk::Box,
    index: ChoiceIndex,
) {
    let box_children: Vec<Widget> = choice_buttons_box.iter_children().collect();
    let default_btn_widget = &box_children[index];
    let default_btn = &default_btn_widget.first_child().unwrap();
    gtk::prelude::RootExt::set_focus(window, Some(default_btn));
}

fn chooser_connect_cancel(sender: ComponentSender<UchooseApp>, window: gtk::ApplicationWindow) {
    // Abort when press esc
    let event_controler = gtk::EventControllerKey::new();
    event_controler.connect_key_pressed(clone!(
        #[strong]
        sender,
        move |_this: &gtk::EventControllerKey,
              keyval: gtk::gdk::Key,
              _keycode: u32,
              _state: gtk::gdk::ModifierType| {
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
