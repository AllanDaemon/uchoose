#pragma once

#include <QMainWindow>

#ifndef FONT_SIZE
#define FONT_SIZE 14
#endif

namespace Ui {
class gui_qt;
}

class gui_qt : public QMainWindow
{
    Q_OBJECT

public:
//    explicit gui_qt(QWidget *parent = nullptr);
    explicit gui_qt(const char *url, QWidget *parent = nullptr);
    ~gui_qt();

    QString url;
private:
    Ui::gui_qt *ui;

    void setupUi();
};

