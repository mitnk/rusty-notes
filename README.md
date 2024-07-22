# Rusty Notes

A local simple **Markdown** web interface.

![rusty-notes](assets/img/rusty-notes-ss.png)

## Install & Setup

```
$ cargo install -f rusty-notes
```

You need to a one-tiime setup setup like:
```
$ git clone https://github.com/mitnk/rusty-notes
$ cd rusty-notes
$ make setup DIR=~/rusty-notes
```

This will make a notebook for your at `~/rusty-notes`, you can make more
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
