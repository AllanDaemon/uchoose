pub type Chooser =
    fn(url: String, browser_list: &Vec<crate::providers::BrowserEntry>, default: Choice) -> Choice;
pub type Choice = usize;

pub mod ui_cli;
