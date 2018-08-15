#!/usr/bin/python3

from PySide2 import QtCore, QtWidgets, QtGui

class UOR(QtWidgets.QWidget):
	def __init__(self):
		QtWidgets.QWidget.__init__(self)


if __name__ == "__main__":
	import sys
	app = QtWidgets.QApplication(sys.argv)

	widget = UOR()
	widget.show()

	sys.exit(app.exec_())

