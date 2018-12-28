#!/usr/bin/python3

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

PADDING_SIZE = 0
ICON_SIZE = 32


class Uchoose(Gtk.Window):
	def __init__(self, url, browser_list, default):
		super().__init__()
		self.url = url
		self.browser_list = browser_list
		self.default = default
		self.choice = None

		self.init_ui()

		self.connect('destroy', Gtk.main_quit)
		self.show_all()

	def init_ui(self):
		self.box = Gtk.VBox()
		self.add(self.box)
		self.url_label = Gtk.Label(label=self.url)
		self.box.pack_start(child=self.url_label, expand=True,
							fill=True, padding=PADDING_SIZE)
		icon_theme = Gtk.IconTheme.get_default()
		for i, (name, icon, _, _) in enumerate(self.browser_list):
			icon_pixbuf = icon_theme.load_icon(
				icon_name=icon, size=ICON_SIZE, flags=Gtk.IconLookupFlags(0))
			image = Gtk.Image.new_from_pixbuf(icon_pixbuf)
			label = Gtk.Label(label=name)
			hbox = Gtk.HBox()
			hbox.pack_start(child=image, expand=False, fill=True, padding=0)
			hbox.pack_start(child=label, expand=True, fill=True, padding=0)
			btn = Gtk.Button()
			btn.add(hbox)
			btn.connect('clicked', lambda _, idx=i: self.chosen(idx))
			self.box.pack_start(child=btn, expand=True, fill=True, padding=0)

	def chosen(self, index):
		self.choice = index
		self.destroy()


def gui_gtk3(url, broswer_list, default):
	app = Gtk.Application()
	app.connect('activate', lambda _: Gtk.main())
	window = Uchoose(url, broswer_list, default)
	app.run()
	return window.choice

# vim: ts=4:sw=4:noet
