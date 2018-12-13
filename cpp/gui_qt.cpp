#include "gui_qt.h"
#include "ui_gui_qt.h"

gui_qt::gui_qt(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::gui_qt)
{
    ui->setupUi(this);
}

gui_qt::~gui_qt()
{
    delete ui;
}
