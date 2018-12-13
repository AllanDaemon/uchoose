#include <QApplication>

#include "uchoose.h"
#include "gui_qt.h"

BrowserEntry entries[] = {
	{"Firefox",				"firefox",				"firefox %u"},
	{"Falkon",				"falkon",				"falkon %u"},
	{"Konqueror",			"konqueror",			"konqueror"},
	{"Bookmark Editor",		"bookmarks-organize",	"keditbookmarks"},
	{"Chromium Web Browser","chromium-browser",		"/usr/bin/chromium-browser %U"},
	{"Google Chrome (beta)","google-chrome-beta",	"/usr/bin/google-chrome-beta %U"},
	{"chromium-vaapi",		"chromium-vaapi",		"/usr/bin/chromium-vaapi %U"},
	{nullptr,				nullptr,				nullptr},
};

BrowserList entries_list = {
	{"Firefox",				"firefox",				"firefox %u"},
	{"Falkon",				"falkon",				"falkon %u"},
	{"Konqueror",			"konqueror",			"konqueror"},
	{"Bookmark Editor",		"bookmarks-organize",	"keditbookmarks"},
	{"Chromium Web Browser","chromium-browser",		"/usr/bin/chromium-browser %U"},
	{"Google Chrome (beta)","google-chrome-beta",	"/usr/bin/google-chrome-beta %U"},
	{"chromium-vaapi",		"chromium-vaapi",		"/usr/bin/chromium-vaapi %U"},
};


int main(int argc, char *argv[])
{
	char* url = url_default;

	QApplication a(argc, argv);
	gui_qt w(url, entries_list, 0);
	w.show();

	return a.exec();
}
