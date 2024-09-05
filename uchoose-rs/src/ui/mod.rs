pub type ChoiceIndex = usize;
pub type Choice = Option<ChoiceIndex>;

pub type Chooser = fn(
    url: String,
    browser_list: &Vec<crate::providers::BrowserEntry>,
    default_option: ChoiceIndex,
) -> Choice;

pub mod ui_cli;

pub mod ui_gtk4;
pub mod ui_relm4;
