BIN_FILE=uchoose
DESKTOP_FILE=uchoose.desktop
SYSTEM_PREFIX=/usr/local

install: install-pip-user
remove: remove-pip-user

install-pip-user:
	pip3 install -U --user ./

install-dev-user:
	./setup.py develop --user

remove-pip-user:
	pip3 uninstall uchoose

remove-dev-user:
	./setup.py develop --user --uninstall


install-user: $(BIN_FILE).py $(DESKTOP_FILE)
	ln -sf $(PWD)/$(BIN_FILE).py $(HOME)/bin/$(BIN_FILE)
	ln -sf $(PWD)/$(DESKTOP_FILE) $(HOME)/.local/share/applications/
	kbuildsycoca5

install-system:
	sudo install -v $(PWD)/$(BIN_FILE).py $(SYSTEM_PREFIX)/bin/$(BIN_FILE)
	sudo install -v $(PWD)/$(DESKTOP_FILE) $(SYSTEM_PREFIX)/share/applications/
	kbuildsycoca5

remove-user:
	rm -f $(HOME)/bin/$(BIN_FILE)
	rm -f $(HOME)/.local/share/applications/$(DESKTOP_FILE)
	kbuildsycoca5

remove-system:
	rm -f $(SYSTEM_PREFIX)/bin/$(BIN_FILE)
	rm -f $(SYSTEM_PREFIX)/share/applications/$(DESKTOP_FILE)
	kbuildsycoca5

clean:
	./setup.py clean -a

.PHONY: install remove remove-user remove-system