#ifndef GUI_QT_H
#define GUI_QT_H

#include <QMainWindow>

namespace Ui {
class gui_qt;
}

class gui_qt : public QMainWindow
{
    Q_OBJECT

public:
    explicit gui_qt(QWidget *parent = nullptr);
    ~gui_qt();

private:
    Ui::gui_qt *ui;
};

#endif // GUI_QT_H
