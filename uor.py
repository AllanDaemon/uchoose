#!/usr/bin/python3

import sys

DEFAULT = 0

def is_terminal():
	import os
	return os.isatty(0) and os.isatty(1)

def help():
	print(f"Usage: {sys.argv[0]} URL OPTIONS????")

def main():
	### for DBG
	url = 'http://example.com/this/is.a.url?all=right'
	browser_list = []

	### Init and parsing

	GUI = not is_terminal()
	if "--gui" in sys.argv: GUI = True
	if "--cli" in sys.argv: GUI = False
	if "--no-gui" in sys.argv: GUI = False
	if "--no-cli" in sys.argv: GUI = True

	GUI = True	# for DBG
	GUI = False	# for DBG

	if GUI: from gui_qt import gui_qt as interface
	else:   from cli    import cli    as interface

	### Program

	from providers import get_browser_list
	browser_list = get_browser_list()
	for n,i,e,d in browser_list: print(n, i, d.filename, repr(e), sep='\t') # dbg

	browser = interface(url, browser_list, DEFAULT)
	
	print('-'*40)
	print(f'URL:', url)
	print(f'Browser:', browser)


if __name__ == "__main__":
	main()