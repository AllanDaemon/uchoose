// vim: ts=4:sw=4:noet ft=cpp:

#include "gui_qt.h"
#include <iostream>
#include <stdio.h>

#define FONT_2SIZE (FONT_SIZE*2)

void print_browser_list(BrowserList& browser_list){
	for(BrowserEntry b: browser_list)
		std::cout << b.name << '\n';
}

gui_qt::gui_qt(const char* url,
			   BrowserList& browser_list,
			   int _default = 0,
			   QWidget *parent) :
	QWidget(parent)
{
	this->app = QCoreApplication::instance();
	this->url = QString(url);
	this->browser_list = &browser_list;
	this->_default = _default;

	print_browser_list(browser_list);

	this->init_ui();
	this->show();
}

void gui_qt::init_ui()
{
	QApplication::setFont(QFont(nullptr, FONT_SIZE));

	auto layout = new QVBoxLayout();
	layout->setSpacing(FONT_SIZE);
	layout->setContentsMargins(FONT_2SIZE, FONT_2SIZE, FONT_2SIZE, FONT_2SIZE);
	this->setLayout(layout);

	std::cout << this->url.toStdString();
	auto url_label = new QLabel(this->url);
	auto line = new QFrame();
	line->setFrameShape(QFrame::HLine);

	layout->addSpacing(FONT_SIZE);
	layout->addWidget(url_label);
	layout->addSpacing(FONT_SIZE);
	layout->addWidget(line);
	layout->addSpacing(FONT_SIZE);

	static QSize icon_size(ICON_SIZE);
	// entries list

	int i = -1;
	for(BrowserEntry b: *this->browser_list)
	{
		i++;
		printf("[%i]\tQT: '%s' '%s' '%s'\n", i, b.name, b.icon, b.cmd);

		auto icon = QIcon::fromTheme(b.icon);
//		auto btn = QPushButton(icon, b.name, dynamic_cast<QWidget*>(layout));
		auto btn = QPushButton(icon, b.name, nullptr);
		btn.setIconSize(icon_size);
		btn.setDefault(i == this->_default);
		btn.connect(
				&QPushButton::clicked,
				[this, i](){ this->chosen(i); }
		);

//		this->entries.append
		layout->addWidget(&btn);

	}
}



gui_qt::~gui_qt()
{
}


void gui_qt::chosen(int index)
{
	printf("CHOSEN: %i", index);
}
