#!/usr/bin/env python3

from setuptools import setup

__version__ = '0.2'

setup(
	name = 'uchoose',
	version = __version__,
	description = 'a browser chooser for default action on url openers',
	author = 'Allan Daemon',
	author_email = 'rea.aft@gmail.com',
	license = 'GPL3',
	keywords = ['browser', 'default', 'url opener','freedesktop', 'xdg'],
	project_urls = {
		'Bug Tracker': 'https://github.com/AllanDaemon/uchoose/issues',
		'Source Code': 'https://github.com/AllanDaemon/uchoose',
	},
	zip_safe = False,
	python_requires = '>=3.6',
	packages = ['uchoose'],
	scripts = ['uchoose/uchoose.py'],

	install_requires = ['pyxdg'],
	extras_require = {
		'QT': ['PySide2'],
		'GTK': [], #@FIX
	},
	# package_data = {'': ['*.desktop']},
	data_files = [
		('share/applications', ['org.allandaemon.uchoose.desktop'])
	],
)
