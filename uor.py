#!/usr/bin/python3

DEFAULT = 0

def is_terminal():
	import os
	return os.isatty(0) and os.isatty(1)


def console_interface(url, browser_list, default):
	from cli import cli
	cli(url, browser_list)

def gui_interface(url, browser_list, default):
	from gui_qt import gui_qt
	gui_qt(url, browser_list)


def main():
	url = 'http://example.com/this/is.a.url?all=right'
	browser_list = []

	if is_terminal():
		browser = console_interface(url, browser_list, DEFAULT)
		from gui_qt import gui_qt as interface
	else:
		browser = gui_interface(url, browser_list, DEFAULT)
		from cli import cli as interface
	
	print('-'*40)
	print(f'URL:', url)
	print(f'Browser:', browser)


if __name__ == "__main__":
	main()