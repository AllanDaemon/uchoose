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
	classifiers = [
		'Development Status :: 3 - Alpha',
		'Environment :: Console',
		'Environment :: X11 Applications',
		'Environment :: X11 Applications :: Gnome',
		'Environment :: X11 Applications :: GTK',
		'Environment :: X11 Applications :: KDE',
		'Environment :: X11 Applications :: Qt',
		'Intended Audience :: End Users/Desktop',
		'License :: DFSG approved',
		'License :: Freely Distributable',
		'License :: OSI Approved',
		'License :: OSI Approved :: GNU General Public License v3 (GPLv3)',
		'Natural Language :: English',
		'Operating System :: POSIX',
		'Operating System :: POSIX :: Linux',
		'Programming Language :: Python',
		'Programming Language :: Python',
		'Programming Language :: Python :: 3',
		'Programming Language :: Python :: 3.6',
		'Programming Language :: Python :: 3.7',
		'Programming Language :: Python :: 3.8',
		'Programming Language :: Python :: 3 :: Only',
		'Programming Language :: Python :: Implementation :: CPython',
		'Topic :: Desktop Environment',
		'Topic :: Internet',
		'Topic :: Internet :: WWW/HTTP',
		'Topic :: Internet :: WWW/HTTP :: Browsers',
		'Topic :: Internet :: WWW/HTTP :: Site Management :: Link Checking',
		'Topic :: Utilities',
	],
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
