#!/usr/bin/env python3


def copy_qt(s:str):
	from PySide2 import QtGui
	c:QtGui.QClipboard = QtGui.QGuiApplication.clipboard()
	c.setText(s)
	print("Copied to clipboard using QT")

def copy_xclip(s:str):
	' |xclip -i -r -selection CLIPBOARD'
	print("Copied to clipboard using xclip")
