use super::{Choice, ChoiceIndex};
use crate::providers::BrowserEntry;

pub fn chooser(url: &str, browser_list: &Vec<BrowserEntry>, default_option: ChoiceIndex) -> Choice {
    println!("Open: {}", url);

    for (i, b) in browser_list.iter().enumerate() {
        let prefix = if i == default_option { '*' } else { ' ' };
        println!("  {}{}  {}", prefix, i, b.name)
    }

    get_choice(browser_list)
}

fn get_choice(browser_list: &Vec<BrowserEntry>) -> Option<usize> {
    let mut input_line = String::new();

    // Print prompt
    print!("({}): ", browser_list[0].name);

    // Read line
    let _input_result = std::io::stdin().read_line(&mut input_line);
    if _input_result.is_err() {
        println!("No answer read");
        return get_choice(browser_list);
    } else if _input_result.unwrap() == 0 {
        // EOF -> just exit the program
        return None;
    }

    // Parse number
    if let Ok(choice) = input_line.trim().parse() {
        // Check number range
        if choice >= browser_list.len() {
            println!("Wrong choice! Must be from 0 up to {}", browser_list.len());
            return get_choice(browser_list);
        }

        // Number ok
        return Some(choice);
    } else {
        // Failed parsing number
        println!("Wrong choice (not a number)!");
        return get_choice(browser_list);
    }
}
