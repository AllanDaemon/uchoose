// vim: ts=4:sw=4:noet ft=cpp:

#pragma once

#include <iostream>
#include <QWidget>
#include <QApplication>
#include <QFont>
#include <QVBoxLayout>
#include <QLabel>
#include <QFrame>
#include <QSize>
#include <QIcon>
#include <QPushButton>
#include "uchoose.h"

#ifndef FONT_SIZE
#define FONT_SIZE 14
#endif

#ifndef ICON_SIZE
#define ICON_SIZE 32, 32
#endif


class gui_qt : public QWidget
{
	Q_OBJECT

public:
	explicit gui_qt(const char *url,
					BrowserList& browser_list,
					int _default,
					QWidget *parent = nullptr);
	~gui_qt();

	QCoreApplication* app;
	QString url;
	BrowserList* browser_list;
	int _default = 0;


private:
	void init_ui();
	void chosen(int index);
//	QBoxLayout layout;
};

