#![allow(dead_code)]
#![allow(unused)]

use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

use iced::widget::{button, column, text, Column};
use iced::Center;

/* NOT IMPLEMENTED

I give up on using ICED by now. Too hard, but it's still in heavy development and unstable.
Maybe revisit it after 1.0 is released
 */

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("Iced Open: {}", url);
    unimplemented!();

    // let result = Rc::new(RefCell::new(None));

    // let choose_params = UchooseParams {
    //     url: url.to_string(),
    //     browser_list: browser_list.clone(),
    //     default_option: default_option.clone(),
    //     result: Rc::clone(&result),
    // };

    let app = iced::application("uChoose", UchooseApp::update, UchooseApp::view);

    println!("App will run");
    let iced_result: iced::Result = app.run_with(UchooseApp::new);
    println!("App ran out\niced_result = {iced_result:#?}\n");

    // return *result.borrow();
    None
}

#[derive(Debug, Default)]
struct UchooseApp {}

#[derive(Debug)]
enum UchooseMessage {
    Increment,
}

impl UchooseApp {
    fn new() -> (Self, iced::Task<UchooseMessage>) {
        (UchooseApp {}, iced::Task::none())
    }

    fn update(&mut self, message: UchooseMessage) {}

    fn view(&self) -> Column<UchooseMessage> {
        let url_label = text("URL").size(50);

        column![url_label]
    }
}
