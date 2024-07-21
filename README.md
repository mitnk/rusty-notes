# Rusty Notes

A local simple **Markdown** web interface.

![rusty-notes](assets/img/rusty-notes-ss.png)

## Install & Setup

## Run Server


```bash
export RUSTY_SERVER_ADDR=127.0.0.1:7777
export RUSTY_NOTES_DIR="$HOME/rusty-enotes"
export RUSTY_TEMPLATES_DIR="$HOME/rusty-enotes/static/templates"
nohup rusty-notes > /tmp/rusty-notes.log &
```
