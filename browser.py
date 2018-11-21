#!/usr/bin/usr python3

import os
import re
from path import path

APPLICATIONS_DIRS = ['/usr/share/applications']
APPLICATIONS_DIRS = list(map(path, APPLICATIONS_DIRS))

XDG_CAT_RE = r'Categories=(.*?)WebBrowser;(.*?)'
browser_re = re.compile(XDG_CAT_RE)

def get_brower_list():
	browsers_xdg_files = []
	for DIR in APPLICATIONS_DIRS:
		files = os.listdir(DIR)
		for f_name in files:
			with open(DIR / f_name) as f:
				content  = f.read()
				if browser_re.search(content):
					browsers_xdg_files.append(DIR / f_name)
					print(browsers_xdg_files[0])

get_brower_list()