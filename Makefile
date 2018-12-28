BIN_FILE=uchoose
DESKTOP_FILE=uchoose.desktop


install:
	ln -sf $(PWD)/uchoose.py $(HOME)/bin/$(BIN_FILE)
	ln -sf $(PWD)/$(DESKTOP_FILE) $(HOME)/.local/share/applications/
	kbuildsycoca5

remove:
	rm -f $(HOME)/bin/$(BIN_FILE)
	rm -f $(HOME)/.local/share/applications/$(DESKTOP_FILE)
	kbuildsycoca5

.PHONY: install remove