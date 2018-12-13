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


class gui_qt : public QWidget
{
	Q_OBJECT

public:
	explicit gui_qt(const char *url,
					std::list<BrowserEntry> browser_list,
					int _default,
					QWidget *parent = nullptr);
	~gui_qt();

	QCoreApplication* app;
	QString url;
	std::list<BrowserEntry> browser_list;
	int _default = 0;


private:
	void init_ui();
//	QBoxLayout layout;
};

