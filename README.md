# Rusty Notes

[![Latest Version](https://img.shields.io/crates/v/rusty-notes.svg)](https://crates.io/crates/rusty-notes)

A simple local Web-UI Notebook using Markdown.

![rusty-notes](assets/img/rusty-notes-ss.png)

## Install & Setup

```
$ cargo install -f rusty-notes
```

You need to a one-time setup like:
```
$ git clone https://github.com/mitnk/rusty-notes
$ cd rusty-notes
$ make setup DIR=~/rusty-notes
```

This will make a notebook for you at `~/rusty-notes`, you can make more
markdown notes under this directory. Sub-directories will be treated as
categories.

You can change the root directory, see "run server" section below.

## Run Server

```bash
export RUSTY_SERVER_ADDR=127.0.0.1:7777
export RUSTY_DIR_NOTES="$HOME/rusty-notes"
export RUSTY_DIR_TEMPLATES="$HOME/rusty-notes/static/templates"
nohup rusty-notes > /tmp/rusty-notes.log &
```

## Static files

Static files under `$RUSTY_DIR_NOTES/static/` can be access with URLs like:
- http://127.0.0.1:7777/stc/img/hello.png
  - for file: `static/img/hello.png`
- http://127.0.0.1:7777/code/2024/foo.c
  - for file: `static/code/2024/foo.c`

## Why is this useful?

So that you have a local Markdown notebook tracked and backed up with Git. And
simply run `rusty-notes` behind Nginx for a straightforward blogging system,
eliminating the need for a static HTML builder.
