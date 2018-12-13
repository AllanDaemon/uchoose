#include "gui_qt.h"
#include <iostream>

#define FONT_2SIZE (FONT_SIZE*2)

gui_qt::gui_qt(const char* url, QWidget *parent) :
    QMainWindow(parent)
{
    this->url = QString(url);
    this->init_ui();
    this->show();
}

void gui_qt::init_ui()
{
    auto layout = new QVBoxLayout();
    layout->setSpacing(FONT_SIZE);
    layout->setContentsMargins(FONT_2SIZE, FONT_2SIZE, FONT_2SIZE, FONT_2SIZE);

    std::cout << this->url.toStdString();
    auto url_label = new QLabel(this->url);
    auto line = new QFrame();
    line->setFrameShape(QFrame::HLine);

    layout->addSpacing(FONT_SIZE);
    layout->addWidget(url_label);
    layout->addSpacing(FONT_SIZE);
    layout->addWidget(line);
    layout->addSpacing(FONT_SIZE);

    this->setLayout(layout);
//    this->setCentralWidget(layout);

}

gui_qt::~gui_qt()
{
}

