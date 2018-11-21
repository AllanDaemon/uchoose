#!/usr/bin/env python3

import os
import re
from pathlib import Path

APPLICATIONS_DIRS = ['/usr/share/applications']
APPLICATIONS_DIRS = list(map(Path, APPLICATIONS_DIRS))

XDG_CAT_RE = r'Categories=(.*?)WebBrowser;(.*?)'
browser_re = re.compile(XDG_CAT_RE)

def get_brower_list():
	browsers_xdg_files = []
	for DIR in APPLICATIONS_DIRS:
		for f in DIR.iterdir():
			if not f.is_file(): continue
			if browser_re.search(f.read_text()):
				browsers_xdg_files.append(f)
				print(browsers_xdg_files[-1])

get_brower_list()