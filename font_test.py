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

		self.url = QtWidgets.QLabel("short")
		self.url = QtWidgets.QLabel("very long."*10)
		self.url = QtWidgets.QLabel("http://example.com/this/is.a.url?all=right")

		icon1 = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/firefox.png')
		icon2 = QtGui.QIcon('/usr/share/icons/hicolor/256x256/apps/google-chrome-beta.png')

		self.entries = [
			QtWidgets.QPushButton(icon1, 'Firefox'),
			QtWidgets.QPushButton(icon2, 'Chrome'),
		]

		size = QtCore.QSize(*ICON_SIZE)
		for e in self.entries: e.setIconSize(size)

		self.layout.addWidget(self.url)
		self.layout.addWidget(self.entries[0])
		self.layout.addWidget(self.entries[1])



app = QtWidgets.QApplication()
font = app.font()
# print(font)
# print(font.pixelSize())
# print(font.pointSize())
# print(font.pointSizeF())
#font.setPointSize(FONT_SIZE)
# font = QtGui.QFont(None, 20)
# print(font)
# print(font.pixelSize())
# print(font.pointSize())
# print(font.pointSizeF())
# app.setFont(font)
app.setFont(QtGui.QFont(None, 20))

widget = UOR()
widget.show()

app.exec_()

