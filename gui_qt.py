#!/usr/bin/python3

from PySide2 import QtCore, QtWidgets, QtGui

FONT_SIZE = 14
ICON_SIZE = 32, 32

class UOR(QtWidgets.QWidget):
	def __init__(self, url, browser_list, default):
		QtWidgets.QWidget.__init__(self)
		self.url = url
		self.browser_list = browser_list
		self.default = default
		self.app = QtCore.QCoreApplication.instance()

		self.init_ui()
		self.show()

	def init_ui(self):
		self.app.setFont(QtGui.QFont(None, FONT_SIZE))

		self.layout = QtWidgets.QVBoxLayout()
		self.layout.setSpacing(FONT_SIZE)	# default: 6
		self.layout.setContentsMargins(FONT_SIZE*2, FONT_SIZE*2, FONT_SIZE*2, FONT_SIZE*2)
		self.setLayout(self.layout)

		url = self.url_label = QtWidgets.QLabel(self.url)
		print('Content Margins:', url.getContentsMargins())
		self.layout.addSpacing(FONT_SIZE)
		self.layout.addWidget(self.url_label)
		self.layout.addSpacing(FONT_SIZE*2)


		icon_size = QtCore.QSize(*ICON_SIZE)
		self.entries = []
		for i, (name, icon, _, _) in enumerate(self.browser_list):
			print("QT:", name, icon)

			icon = QtGui.QIcon.fromTheme(icon)
			btn = QtWidgets.QPushButton(icon, name, self)
			btn.setIconSize(icon_size)
			btn.setDefault(i == self.default)

			self.entries.append(btn)
			self.layout.addWidget(btn)

		self.center()

	def center(self):
		desktop = self.app.desktop()
		screen_num = desktop.screenNumber(desktop.cursor().pos())
		screen_center = desktop.screenGeometry(screen_num).center()
		self.adjustSize()
		win = self.frameGeometry()
		win.moveCenter(screen_center)
		self.move(win.topLeft())

def gui_qt(url, browser_list, default):
	app = QtWidgets.QApplication()

	widget = UOR(url, browser_list, default)

	import sys
	sys.exit(app.exec_())
	# return chosen
