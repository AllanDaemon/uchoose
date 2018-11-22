#!/usr/bin/python3

import os
from PySide2 import QtCore, QtWidgets, QtGui

FONT_SIZE = 20
ICON_SIZE = 32, 32

class UOR(QtWidgets.QWidget):
	def __init__(self):
		QtWidgets.QWidget.__init__(self)
		self.layout = QtWidgets.QVBoxLayout()
		self.setLayout(self.layout)

		self.url = QtWidgets.QLabel("http://example.com/this/is.a.url?all=right")
		self.url.setFont(QtGui.QFont(fontSize=20))

		icon1 = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/firefox.png')
		icon2 = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/google-chrome-beta.png')

		self.entries = [
			QtWidgets.QPushButton(icon1, 'Firefox'),
			QtWidgets.QPushButton(icon2, 'Chrome'),
		]

		self.layout.addWidget(self.url)
		self.layout.addWidget(self.entries[0])
		self.layout.addWidget(self.entries[1])


def gui_qt(url, browser_list, default):
# if __name__ == "__main__":
	import sys
	app = QtWidgets.QApplication(sys.argv)

	widget = UOR()
	widget.show()

	sys.exit(app.exec_())
	# return chosen

