BIN_FILE=uchoose
DESKTOP_FILE=uchoose.desktop
SYSTEM_PREFIX=/usr/local

install: install-user

install-user: $(BIN_FILE).py $(DESKTOP_FILE)
	ln -sf $(PWD)/$(BIN_FILE).py $(HOME)/bin/$(BIN_FILE)
	ln -sf $(PWD)/$(DESKTOP_FILE) $(HOME)/.local/share/applications/
	kbuildsycoca5

install-system:
	sudo install -v $(PWD)/$(BIN_FILE).py $(SYSTEM_PREFIX)/bin/$(BIN_FILE)
	sudo install -v $(PWD)/$(DESKTOP_FILE) $(SYSTEM_PREFIX)/share/applications/
	kbuildsycoca5


remove: remove-user

remove-user:
	rm -f $(HOME)/bin/$(BIN_FILE)
	rm -f $(HOME)/.local/share/applications/$(DESKTOP_FILE)
	kbuildsycoca5

remove-system:
	rm -f $(SYSTEM_PREFIX)/bin/$(BIN_FILE)
	rm -f $(SYSTEM_PREFIX)/share/applications/$(DESKTOP_FILE)
	kbuildsycoca5
	

.PHONY: install remove remove-user remove-system