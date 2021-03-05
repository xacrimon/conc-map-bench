#!/usr/bin/env bash

set -x

BIN=./target/release/conc-map-bench
OUT=./results

cargo build --release
mkdir -p "$OUT"

function bench {
    ARGS=$3

    if [ "$2" = "std" ]; then
        ARGS+=" --use-std-hasher"
    fi

    date

    file="$OUT/$1.$2.csv"

    if [ -s "$file" ]; then
        ARGS+=" --csv-no-headers"
    fi

    skip=$(cat "$file" | cut -d, -f1 | uniq | paste -sd ' ' -)

    if ! "$BIN" bench -w $1 $ARGS --skip $skip --csv 2>>"$file"; then
        bench "$1" "$2" "$3"
    fi
}

bench ReadHeavy fx
bench Exchange fx
bench RapidGrow fx

bench ReadHeavy std
bench Exchange std
bench RapidGrow std
date
