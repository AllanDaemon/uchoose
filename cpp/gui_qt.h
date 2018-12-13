#pragma once

#include <iostream>
#include <QWidget>
#include <QVBoxLayout>
#include <QLabel>
#include <QFrame>
#include <QSize>
#include <QIcon>
#include <QPushButton>

#ifndef FONT_SIZE
#define FONT_SIZE 14
#endif


class gui_qt : public QWidget
{
	Q_OBJECT

public:
	explicit gui_qt(const char *url, QWidget *parent = nullptr);
	~gui_qt();

	QString url;

private:
	void init_ui();
//    QBoxLayout layout;
};

