#!/bin/bash
set -eu

_die() {
    echo "err: $*"
    exit 1
}


if which docker-compose >/dev/null; then
    COMPOSE="docker-compose"
elif docker compose >/dev/null; then
    COMPOSE="docker compose"
else
    echo "could not locate docker compose command or plugin"
    exit 1
fi
BCLI="$COMPOSE exec -T -u blits bitcoind bitcoin-cli -regtest"
DATA_DIR="./srv"

start() {
    $COMPOSE down -v
    rm -rf $DATA_DIR
    mkdir -p $DATA_DIR
    $COMPOSE up -d

    # wait for bitcoind to be up
    until $COMPOSE logs bitcoind |grep 'Bound to'; do
        sleep 1
    done

    # prepare bitcoin funds
    $BCLI createwallet miner
    $BCLI -rpcwallet=miner -generate 103

    # wait for electrs to have completed startup
    until $COMPOSE logs electrs |grep 'finished full compaction'; do
        sleep 1
    done

    # wait for proxy to have completed startup
    until $COMPOSE logs proxy |grep 'App is running at http://localhost:3000'; do
        sleep 1
    done
}

stop() {
    $COMPOSE down -v
    rm -rf $DATA_DIR
}

[ -n "$1" ] || _die "command required"
case $1 in
    start|stop) "$1";;
    *) _die "unrecognized command";;
esac