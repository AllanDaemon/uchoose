#!/usr/bin/python3

from sys import argv

DEFAULT = 0

def is_terminal():
	import os
	return os.isatty(0) and os.isatty(1)

def help():
	print(f"Usage: {argv[0]} URL OPTIONS????")

def main():
	### for DBG
	url = 'http://example.com/this/is.a.url?all=right'
	browser_list = []

	### Init and parsing

	if len(argv) > 1:
		for arg in argv[1:]:
			if not arg.startswith('--'):
				url = arg
				break

	GUI = not is_terminal()
	if "--gui" in argv: GUI = True
	if "--cli" in argv: GUI = False
	if "--no-gui" in argv: GUI = False
	if "--no-cli" in argv: GUI = True

	# GUI = False	# for DBG
	# GUI = True	# for DBG

	if GUI: from gui_qt import gui_qt as interface
	else:   from cli    import cli    as interface

	### Program

	from providers import get_browser_list
	browser_list = get_browser_list()
	#for n,i,e,d in browser_list: print(n, i, d.filename, repr(e), sep='\t') # dbg

	choice, browser = interface(url, browser_list, DEFAULT)
	
	print('-'*40)
	print(f'URL:      ', url)
	print(f'BROWSER:  ', choice, repr(browser.name))


if __name__ == "__main__":
	main()