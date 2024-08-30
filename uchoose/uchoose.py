#!/usr/bin/env python3

import argparse
import os
import sys
from sys import exit
import shlex

DEFAULT = 0
FORK = True

UI_LIST = (
	'cli',	# Simple cli
	'qt5',
	'qt6',
	'gtk3',
)


# https://developer.gnome.org/integration-guide/stable/desktop-files.html.en#tb-exec-params
def param_subs(arg:str, url:str) -> str:
	if r'%u' in arg: arg = arg.replace(r'%u', url)
	if r'%U' in arg: arg = arg.replace(r'%U', url)
	if r'%k' in arg: arg = arg.replace(r'%k', url)
	return arg

def execute(name:str, exec_cmd:str, url:str):
	print(f"EXECUTING  {exec_cmd!r} @ {url!r}")

	cmd = shlex.split(exec_cmd)
	print("	PRE:", repr(cmd))
	cmd = [ param_subs(arg, url) for arg in cmd]
	if r'%u' not in exec_cmd.lower(): cmd.append(url)
	print("	CMD:", repr(cmd))
	return exec_(cmd)

def exec_(cmd, fork=FORK):
	print("EXEC_", cmd)

	if not fork:
		os.execvp(cmd[0], cmd)
		# exec never returns
		assert False, "BUG"
	else:
		os.spawnvp(os.P_NOWAIT, cmd[0], cmd)
	exit()

def copy2clipboard(s: str):
	from .clipboard import copy
	copy(s)


### Main functions

def is_terminal() -> bool:
	import os
	return os.isatty(0) and os.isatty(1)

def _figure_out_ui() -> str:
	if is_terminal():
		return _figure_out_cli()
	else:
		return _figure_out_gui()

def _figure_out_gui() -> str:
	#@TODO: Check and decide between QT and KDE
	return 'qt6'

def _figure_out_cli() -> str:
	# When implement ncurses, check best option
	return 'cli'


def _parse_args_and_settings():
	dbg_url = 'http://example.com/this/is.a.url?all=right'
	description = 'Act as default browser, letting you choose what will open an url'

	### Init and parsing
	sys.UI_SCALE = float(os.environ.get('UI_SCALE', 1.0))

	parser = argparse.ArgumentParser(description=description)
	grp = parser.add_mutually_exclusive_group()
	grp.add_argument('-c', '--cli', const='cli', action='store_const', dest='ui', help='a simple command line interface')
	grp.add_argument('-q', '--qt6', const='qt6', action='store_const', dest='ui', help='QT6 interface')
	grp.add_argument('-5', '--qt5', const='qt5', action='store_const', dest='ui', help='QT5 interface')
	grp.add_argument('-g', '--gtk3', const='gtk3', action='store_const', dest='ui', help='GTK3 interface')
	grp.add_argument('-u', '--ui', choices=UI_LIST, dest='ui', help='select an user interface')
	parser.add_argument('url', default=dbg_url, nargs='?')
	args = parser.parse_args()

	url = args.url
	ui = args.ui

	if not ui:
		if 'UI' in os.environ:
			ui = os.environ['UI'].lower()
			assert ui in UI_LIST
		else:
			ui = _figure_out_ui()

	# ui = 'cli'
	# ui = 'qt5'
	# ui = 'qt6'
	# ui = 'gtk3'

	return url, ui


def main():
	url, ui = _parse_args_and_settings()

	if   ui == 'cli':
		from .ui_cli import chooser
	elif ui == 'qt6':
		from .ui_qt6 import chooser
	elif ui == 'qt5':
		from .ui_qt5 import chooser
	elif ui == 'gtk3':
		from .ui_gtk3 import chooser
	else:
		RuntimeError(f"Unknown user interface: {ui}")



	### Load providers (browser list)

	from .providers import get_browser_list, clipboard_entry
	browser_list = get_browser_list()
	#for n,i,e,d in browser_list: print(n, i, d.filename, repr(e), sep='\t') # dbg
	# for n,i,e,d in browser_list: print(n, i, repr(e), sep='\t') # dbg

	########
	#for n,_,e,_ in browser_list: execute(n, e, url)
	#exit()
	########

	### Choose an entry from browser list

	try:
		choice = chooser(url, browser_list, DEFAULT)
	except (KeyboardInterrupt, EOFError):
		exit(0)
	if choice is None: exit()
	browser = browser_list[choice]

	print('-'*40)
	print(f'URL:      ', url)
	print(f'BROWSER:  ', choice, repr(browser.name))

	### Execute the action

	if browser == clipboard_entry:
		copy2clipboard(url)
	else:
		execute(browser.name, browser.cmd, url)


if __name__ == "__main__":
	main()

# vim: ts=4:sw=4:noet
