#include <QApplication>

#include "uchoose.h"
#include "gui_qt.h"

int main(int argc, char *argv[])
{
	char* url = url_default;
	QApplication a(argc, argv);
	gui_qt w(url);
	w.show();

	return a.exec();
}
