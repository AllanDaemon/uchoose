use std::io;

use crate::providers::BrowserEntry;

use super::Choice;

fn get_choice(browser_list: &Vec<BrowserEntry>) -> Result<usize, ()> {
    let mut input_line = String::new();

    print!("({}): ", browser_list[0].name);

    if io::stdin().read_line(&mut input_line).is_err() {
        println!("No answer read");
        return get_choice(browser_list);
    }
    if let Ok(choice) = input_line.trim().parse() {
        if choice < 0 || choice >= browser_list.len() {
            println!("Wrong choice! Must be from 0 up to {}", browser_list.len());
            return get_choice(browser_list);
        }
        return Ok(choice);
    } else {
        println!("Wrong choice (not a number)!");
        return get_choice(browser_list);
    }
}

pub fn chooser(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("Open: {}", url);

    for (i, b) in browser_list.iter().enumerate() {
        let prefix = if i == default { '*' } else { ' ' };

        println!("  {}{}  {}", prefix, i, b.name)
    }

    if let Ok(choice) = get_choice(browser_list) {
		choice
	} else {
		println!("Error getting choice; using default");
		default
	}
}
