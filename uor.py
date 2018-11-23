#!/usr/bin/python3

import argparse
import os
from sys import argv
import shlex

DEFAULT = 0
FORK = True


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




def is_terminal() -> bool:
	import os
	return os.isatty(0) and os.isatty(1)

def help():
	print(f"Usage: {argv[0]} URL OPTIONS????")

def main():
	### for DBG
	url = 'http://example.com/this/is.a.url?all=right'
	browser_list = []

	### Init and parsing

	GUI = not is_terminal()

	parser = argparse.ArgumentParser()
	grp = parser.add_mutually_exclusive_group()
	grp.add_argument('--gui', action='store_true', default=GUI)
	grp.add_argument('--cli', action='store_false', dest='gui')
	parser.add_argument('url', default=url, nargs='?')

	args = parser.parse_args()
	url = args.url
	GUI = args.gui

	# GUI = False	# for DBG
	# GUI = True	# for DBG

	if GUI: from gui_qt import gui_qt as chooser
	else:   from cli    import cli    as chooser

	### Program

	#@TODO: put a copy to clipboard option
	from providers import get_browser_list
	browser_list = get_browser_list()
	#for n,i,e,d in browser_list: print(n, i, d.filename, repr(e), sep='\t') # dbg
	# for n,i,e,d in browser_list: print(n, i, repr(e), sep='\t') # dbg

	########
	#for n,_,e,_ in browser_list: execute(n, e, url)
	#exit()
	########

	choice = chooser(url, browser_list, DEFAULT)
	if choice is None: exit()
	browser = browser_list[choice]

	print('-'*40)
	print(f'URL:      ', url)
	print(f'BROWSER:  ', choice, repr(browser.name))

	execute(browser.name, browser.cmd, url)


if __name__ == "__main__":
	main()

# vim: ts=4:sw=4:noet
