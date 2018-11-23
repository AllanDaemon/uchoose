#!/usr/bin/python3

import os
from PySide2 import QtCore, QtWidgets, QtGui

FONT_SIZE = 20
ICON_SIZE = 32, 32


class UOR(QtWidgets.QWidget):
	def __init__(self):
		QtWidgets.QWidget.__init__(self)
		self.app = QtCore.QCoreApplication.instance()
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

		self.center()

	def center(self):
		desktop = self.app.desktop()
		# print('availableGeometry:\t', desktop.availableGeometry())
		# print('availableGeometry(0):\t', desktop.availableGeometry(0))
		# print('availableGeometry(1):\t', desktop.availableGeometry(1))
		# print('geometry:\t\t', desktop.screenGeometry())
		# print('geometry(0):\t\t', desktop.screenGeometry(0))
		# print('geometry(1):\t\t', desktop.screenGeometry(1))
		print('frameGeometry:\t', self.frameGeometry())
		print('geometry:\t', self.geometry())
		print('rect:\t\t', self.rect())
		self.adjustSize()
		print('frameGeometry:\t', self.frameGeometry())
		print('rect:\t\t', self.rect())

		pos = desktop.cursor().pos()
		screen_num = desktop.screenNumber(pos)
		screen_geometry = desktop.screenGeometry(screen_num)
		screen_center = desktop.screenGeometry(screen_num).center()
		# --- https://forum.qt.io/topic/21035/how-to-move-the-window-to-the-center-of-the-screen/3
		# self.move(screen_center - self.rect().center())
		# --- https://wiki.qt.io/How_to_Center_a_Window_on_the_Screen
		# new_geometry = QtWidgets.QStyle.alignedRect(
		# 	QtCore.Qt.LeftToRight,
		# 	QtCore.Qt.AlignCenter,
		# 	self.size(),
		# 	screen_geometry)
		# self.setGeometry(new_geometry)
		# --- https://gist.github.com/saleph/163d73e0933044d0e2c4
		# --- https://stackoverflow.com/questions/20243637/pyqt4-center-window-on-active-screen
		win = self.frameGeometry() # This should be bigger. but it's not what is happening
		win = self.geometry()
		win.moveCenter(screen_center)
		self.move(win.topLeft())









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

# vim: sw=4:ts=4:noet
