#!/usr/bin/env python3

from collections import namedtuple
from pathlib import Path

import xdg
from xdg.DesktopEntry import DesktopEntry

APPLICATIONS_DIRS = ['/usr/share/applications']
APPLICATIONS_DIRS = list(map(Path, APPLICATIONS_DIRS))

BrowserEntry = namedtuple('BrowserEntry', ['name', 'icon', 'exec', 'de'])


def get_browser_list():
	de_list = get_browser_desktop_list()
	return [ BrowserEntry(de.getName(), de.getIcon(), de.getExec(), de)
		for de in de_list ]

def get_browser_desktop_list():
	browsers_xdg_files = []

	for DIR in APPLICATIONS_DIRS:
		for f in DIR.iterdir():
			if not f.is_file(): continue
			try: de = DesktopEntry(f)
			except xdg.Exceptions.ParsingError: continue
			if 'WebBrowser' in de.getCategories():
				browsers_xdg_files.append(de)
	return browsers_xdg_files


if __name__=='__main__':
	l = get_browser_list()
	for n,i,e,d in l: print(n, i, d.filename, e, sep='\t')
	name,icon,de = l[0]

__all__ = ['get_browser_list']
