#include "gui_qt.h"
#include "ui_gui_qt.h"


gui_qt::gui_qt(const char* url, QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::gui_qt)
{
    this->url = QString(url);

    ui->setupUi(this);
}

gui_qt::~gui_qt()
{
    delete ui;
}

void gui_qt::setupUi(){
}
