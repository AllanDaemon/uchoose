use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

use iced::widget::{button, column, text, Column};
use iced::Center;

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("Iced Open: {}", url);

    // let result = Rc::new(RefCell::new(None));

    // let choose_params = UchooseParams {
    //     url: url.to_string(),
    //     browser_list: browser_list.clone(),
    //     default_option: default_option.clone(),
    //     result: Rc::clone(&result),
    // };

    println!("App will run");
    let iced_result: iced::Result = iced::run("uChoose", UchooseApp::update, UchooseApp::view);
    println!("App ran out\n");

    // return *result.borrow();
    None
}

#[derive(Debug, Default)]
struct UchooseApp {}

#[derive(Debug)]
struct UchooseMessage {}

fn update(&mut self, message: UchooseMessage) {}
impl UchooseApp {
    fn view(&self) -> Column<UchooseMessage> {
        column![]
    }
}
