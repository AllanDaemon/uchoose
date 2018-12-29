#!/usr/bin/env python3

#@TODO: Use colors/bold
#@TODO: use ncurses or something alike

def get_choice(browser_list):
	choice = input(f'({browser_list[0].name}): ')

	if not choice: return None

	try:
		choice = int(choice)
	except:
		return get_choice(browser_list)

	if choice < 0 or choice >= len(browser_list):
		return get_choice(browser_list)

	return choice



def cli(url, browser_list, default=0):
	print(f"Open: {url}")

	for i, b in enumerate(browser_list):
		print(f"  {'*' if i==default else ' '}{i}  {b.name}")

	choice = get_choice(browser_list)
	if choice is None: choice = default

	return choice

# vim: ts=4:sw=4:noet
