.PHONY: setup

setup:
	@if [ -z "$(DIR)" ]; then \
		echo "Usage: make setup DIR=<directory>"; \
		exit 1; \
	fi
	mkdir -p $(DIR)/math/
	ln -sf $(realpath ./assets/hello-world.md) $(DIR)/
	ln -sf $(realpath ./assets/math-000.md) $(DIR)/math/000.md
	mkdir -p $(DIR)/static/
	ln -sf $(realpath ./assets/css) $(DIR)/static/
	ln -sf $(realpath ./assets/js) $(DIR)/static/
	ln -sf $(realpath ./assets/img) $(DIR)/static/
	ln -sf $(realpath ./assets/templates) $(DIR)/static/
