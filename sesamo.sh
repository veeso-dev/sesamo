#!/bin/bash

cd "$(dirname "$0")" || exit 1

OP="$1"

ENV=".env"
set -a
. $ENV
set +a

if [ -z "$PIDFILE" ]; then
  echo "pidfile must be specified"
  exit 255
fi

if [ -z "$HOME" ]; then
  export HOME=/root
fi

CARGO=$(which cargo)
if [ -z "$CARGO" ]; then
  export CARGO_DIR="/$HOME/.cargo"
  [ -s "$CARGO_DIR/env" ] && \. "$CARGO_DIR/env"  # This loads nvm
fi

start() {
  CMD=$(which sesamo)
  if [ -z "$CMD" ]; then
    CMD="cargo make -p production run"
  fi

  screen -S sesamo -d -m $CMD

  return $?
}

stop() {
  PID=$(cat $PIDFILE)

  kill "$PID"

  return $?
}

case "$1" in

  "start")
    start
    ;;
  
  "stop")
    stop
    ;;
  
  *)
    "unknown operation $OP"
    exit 1
    ;;

esac