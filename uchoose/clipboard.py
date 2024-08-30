#!/usr/bin/env python3

# WORKING
def copy_pyperclip(s: str):
	from pyperclip import copy
	copy(s)
	print("Clipboard [pyperclip] <-", s)

# NOT WORKING (HANGING)
def copy_tk(s: str):
	from tkinter import Tk
	r = Tk()
	r.withdraw()
	r.clipboard_clear()
	r.clipboard_append(s)
	r.update()
	# r.destroy()
	print("Clipboard [tkinter] <-", s)

# NOT WORKING (HANGING)
def copy_qt5(s:str):
	from PySide2 import QtGui

	app: QtGui.QGuiApplication = QtGui.QGuiApplication.instance()
	if not app: app = QtGui.QGuiApplication([])
	print("DBG: QT APP:", app)
	c:QtGui.QClipboard = app.clipboard()
	c.setText(s)
	print("Clipboard [QT5 (PySide2)] <-", s)

# TODO: Test it
def copy_qt6(s:str):
	from PySide6 import QtGui

	app: QtGui.QGuiApplication = QtGui.QGuiApplication.instance()
	if not app: app = QtGui.QGuiApplication([])
	print("DBG: QT APP:", app)
	c:QtGui.QClipboard = app.clipboard()
	c.setText(s)
	print("Clipboard [QT5 (PySide2)] <-", s)

# NOT IMPLEMENTED
def copy_gtk(s: str):
	print("Clipboard [GTK] <-", s)

copy = copy_pyperclip