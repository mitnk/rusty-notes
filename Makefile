notes-dir:
	mkdir -p ~/rusty-notes/math/
	ln -sf $(realpath ./assets/hello-world.md) ~/rusty-notes/
	ln -sf $(realpath ./assets/math-000.md) ~/rusty-notes/math/000.md
	mkdir -p ~/rusty-notes/static/
	ln -sf $(realpath ./assets/css) ~/rusty-notes/static/
	ln -sf $(realpath ./assets/js) ~/rusty-notes/static/
	ln -sf $(realpath ./assets/img) ~/rusty-notes/static/
	ln -sf $(realpath ./assets/templates) ~/rusty-notes/static/
