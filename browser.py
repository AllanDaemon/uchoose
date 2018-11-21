#!/usr/bin/env python3

import os
import re
from pathlib import Path

import xdg
from xdg.DesktopEntry import DesktopEntry

APPLICATIONS_DIRS = ['/usr/share/applications']
APPLICATIONS_DIRS = list(map(Path, APPLICATIONS_DIRS))

# XDG_CAT_RE = r'Categories=(.*?)WebBrowser;(.*?)'
# browser_re = re.compile(XDG_CAT_RE)

def get_browser_list():
	de_list = get_browser_desktop_list()
	return [ (de.getName(), de.getIcon(), de)
		for de in de_list.values() ]

def get_browser_desktop_list():
	browsers_xdg_files = {}

	for DIR in APPLICATIONS_DIRS:
		for f in DIR.iterdir():
			if not f.is_file(): continue
			try: de = DesktopEntry(f)
			except xdg.Exceptions.ParsingError: continue

			if 'WebBrowser' in de.getCategories():
				browsers_xdg_files[f] = de
				print(f)
	return browsers_xdg_files

# def get_browser_xdg_files_list():
# 	browsers_xdg_files = []
# 	for DIR in APPLICATIONS_DIRS:
# 		for f in DIR.iterdir():
# 			if not f.is_file(): continue
# 			try: de = DesktopEntry(f)
# 			except xdg.Exceptions.ParsingError: continue
# 			if 'WebBrowser' in de.getCategories():
# 			# if browser_re.search(f.read_text()):
# 				browsers_xdg_files.append([f, de])
# 				print(browsers_xdg_files[-1])
# 	return browsers_xdg_files


l = get_browser_list()
# ff = list(l.values())[0]

