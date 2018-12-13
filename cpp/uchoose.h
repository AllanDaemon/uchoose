#pragma once

#ifdef __cplusplus
#include <vector>

extern "C" {
#endif


static char url_default[] = "http://example.com/this/is.a.url?all=right";


struct BrowserEntry {
	const char* name;
	const char* icon;
	const char* cmd;
//	char* de;
};

extern BrowserEntry entries[];

#ifdef __cplusplus
using BrowserList = std::vector<BrowserEntry>;

extern BrowserList entries_list;

}
#endif
