#!/usr/bin/python3

def cli(url, browser_list, default=0):
	print(f"Open: {url}")
	# browser_list_ex = ['Firefox', 'Chrome']
	for i, b in enumerate(browser_list):
		print(f"\t{i}  {b.name}")
	choice = input(f'({browser_list[0].name}): ')
	print('CHOICE:', repr(choice))
	
	if choice: choice = int(choice)
	else: choice = default

	return choice, browser_list[choice]