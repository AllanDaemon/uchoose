#!/usr/bin/python3

from PySide2 import QtCore, QtWidgets, QtGui

FONT_SIZE = 20
ICON_SIZE = 32, 32

class UOR(QtWidgets.QWidget):
	def __init__(self, url, browser_list, default):
		QtWidgets.QWidget.__init__(self)
		self.url = url
		self.browser_list = browser_list
		self.default = default

		self.init_ui()
		self.show()

	def init_ui(self):
		self.layout = QtWidgets.QVBoxLayout()
		self.setLayout(self.layout)

		self.url = QtWidgets.QLabel(self.url)
		self.layout.addWidget(self.url)

		self.entries = []
		for name, icon, _, _ in self.browser_list:
			print("QT:", name, icon)

			icon = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/firefox.png')
			icon = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/google-chrome-beta.png')

			btn = QtWidgets.QPushButton(icon, name, self)

			self.entries.append(btn)
			self.layout.addWidget(btn)


def gui_qt(url, browser_list, default):
	app = QtWidgets.QApplication()

	widget = UOR(url, browser_list, default)

	import sys
	sys.exit(app.exec_())
	# return chosen
