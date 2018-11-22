#!/usr/bin/python3

import os
from PySide2 import QtCore, QtWidgets, QtGui

def is_terminal():
	import os
	return os.isatty(0) and os.isatty(1)

class UOR(QtWidgets.QWidget):
	def __init__(self):
		QtWidgets.QWidget.__init__(self)
		self.layout = QtWidgets.QVBoxLayout()
		self.setLayout(self.layout)

		self.l0 = QtWidgets.QLabel(f"  0:  {os.isatty(0)}    ")
		self.l1 = QtWidgets.QLabel(f"  1:  {os.isatty(1)}    ")
		self.l2 = QtWidgets.QLabel(f"  2:  {os.isatty(2)}    ")
		self.l3 = QtWidgets.QLabel(f"  3:  {os.isatty(3)}    ")

		self.layout.addWidget(self.l0)
		self.layout.addWidget(self.l1)
		self.layout.addWidget(self.l2)
		self.layout.addWidget(self.l3)


if __name__ == "__main__":
	import sys
	app = QtWidgets.QApplication(sys.argv)

	widget = UOR()
	widget.show()

	sys.exit(app.exec_())

