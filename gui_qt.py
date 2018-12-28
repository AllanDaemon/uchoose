#!/usr/bin/python3

import signal
from PySide2 import QtCore, QtWidgets, QtGui

FONT_SIZE = 14
ICON_SIZE = 32, 32

class Uchoose(QtWidgets.QWidget):
	def __init__(self, url, browser_list, default):
		QtWidgets.QWidget.__init__(self)
		self.url = url
		self.browser_list = browser_list
		self.default = default
		self.app = QtCore.QCoreApplication.instance()
		self.choice = None

		self.init_ui()
		self.show()

	def init_ui(self):
		self.app.setFont(QtGui.QFont(None, FONT_SIZE))

		self.layout = QtWidgets.QVBoxLayout()
		self.layout.setSpacing(FONT_SIZE)	# default: 6
		self.layout.setContentsMargins(FONT_SIZE*2, FONT_SIZE*2, FONT_SIZE*2, FONT_SIZE*2)
		self.setLayout(self.layout)

		self.url_label = QtWidgets.QLabel(self.url)
		# self.url_label.setFrameStyle(QtWidgets.QFrame.Box)
		# self.url_label.setFrameStyle(QtWidgets.QFrame.StyledPanel)
		line = QtWidgets.QFrame()
		line.setFrameShape(QtWidgets.QFrame.HLine)

		self.layout.addSpacing(FONT_SIZE)
		self.layout.addWidget(self.url_label)
		self.layout.addSpacing(FONT_SIZE)
		self.layout.addWidget(line)
		self.layout.addSpacing(FONT_SIZE)


		icon_size = QtCore.QSize(*ICON_SIZE)
		self.entries = []
		for i, (name, icon, _, _) in enumerate(self.browser_list):
			print("QT:", i, name, icon)

			icon = QtGui.QIcon.fromTheme(icon)
			btn = QtWidgets.QPushButton(icon, name, self)
			btn.setIconSize(icon_size)
			btn.setDefault(i == self.default)
			btn.clicked.connect(lambda state=None, idx=i: self.chosen(idx))

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

	def keyPressEvent(self, event):
		print('EV:', event.text(), event)
		if event.matches(QtGui.QKeySequence.Cancel):
			print("Key CANCEL")
			self.app.exit()
		if event.key() == QtCore.Qt.Key_Q:
			print("KEY Q")
			self.app.exit()

	def chosen(self, index):
		print("CHOSEN", index)
		self.choice = index
		self.app.quit()



# https://stackoverflow.com/questions/4938723/what-is-the-correct-way-to-make-my-pyqt-application-quit-when-killed-from-the-co
class Application(QtWidgets.QApplication):
	'Allow catch SIGTERM and etc'
	def event(self, e):
		return QtWidgets.QApplication.event(self, e)

def gui_qt(url, browser_list, default):
	# app = QtWidgets.QApplication()
	app = Application()

	def quit_handler(*args): print('BYE'); app.quit()
	signal.signal(signal.SIGINT, quit_handler)
	signal.signal(signal.SIGTERM, quit_handler)

	widget = Uchoose(url, browser_list, default)

	app.exec_()
	return widget.choice

# vim: ts=4:sw=4:noet
