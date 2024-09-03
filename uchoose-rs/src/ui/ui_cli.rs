use super::Choice;
use crate::providers::BrowserEntry;

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

pub fn chooser(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("Open: {}", url);

    for (i, b) in browser_list.iter().enumerate() {
        let prefix = if i == default { '*' } else { ' ' };
        println!("  {}{}  {}", prefix, i, b.name)
    }

    if let Some(choice) = get_choice(browser_list) {
        choice
    } else {
        println!("Error getting choice; using default");
        default
    }
}
