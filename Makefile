.PHONY: setup

setup:
	@if [ -z "$(DIR)" ]; then \
		echo "Usage: make setup DIR=<directory>"; \
		exit 1; \
	fi
	mkdir -p $(DIR)/math/
	ln -s $(realpath ./assets/hello-world.md) $(DIR)/ | cat
	ln -s $(realpath ./assets/math-000.md) $(DIR)/math/000.md | cat
	mkdir -p $(DIR)/static/css/
	ln -s $(realpath ./assets/css/notes.css) $(DIR)/static/css/ | cat
	ln -s $(realpath ./assets/css/pure-min.css) $(DIR)/static/css/ | cat
	ln -s $(realpath ./assets/css/pygments.css) $(DIR)/static/css/ | cat
	ln -s $(realpath ./assets/css/syntect.css) $(DIR)/static/css/ | cat
	mkdir -p $(DIR)/static/js/
	ln -s $(realpath ./assets/js/notes.js) $(DIR)/static/js/ | cat
	mkdir -p $(DIR)/static/img/
	ln -s $(realpath ./assets/img/rusty-notes.png) $(DIR)/static/img/ | cat
	mkdir -p $(DIR)/static/templates/
	ln -s $(realpath ./assets/templates/notes) $(DIR)/static/templates/ | cat
