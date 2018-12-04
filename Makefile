BIN_FILE=uor
DESKTOP_FILE=uor.desktop


install:
	ln -sf $(PWD)/uor.py $(HOME)/bin/$(BIN_FILE)
	ln -sf $(PWD)/$(DESKTOP_FILE) $(HOME)/.local/share/applications/

remove:
	rm -f $(HOME)/bin/$(BIN_FILE)
	rm -f $(HOME)/.local/share/applications/$(DESKTOP_FILE)

.PHONY: install remove