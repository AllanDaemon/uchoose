use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

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
    println!("App ran out\n");

    // return *result.borrow();
    None
}
