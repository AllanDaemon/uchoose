// fn get_choice(browser_list):
// 	choice = input(f'({browser_list[0].name}): ')

// 	if not choice: return None

// 	try:
// 		choice = int(choice)
// 	except:
// 		return get_choice(browser_list)

// 	if choice < 0 or choice >= len(browser_list):
// 		return get_choice(browser_list)

// 	return choice

use crate::providers::BrowserEntry;

use super::Choice;

pub fn chooser(url: String, browser_list: &Vec<BrowserEntry>, default: Choice) -> Choice {
    println!("Open: {}", url);

    // for i, b in enumerate(browser_list):
    // 	print(f"  {'*' if i==default else ' '}{i}  {b.name}")

    // choice = get_choice(browser_list)
    // if choice is None: choice = default

    // choice
    return 0;
}
