#!/usr/bin/env bash

BIN=./target/release/conc-map-bench
DATA_DIR=results

DATA_FILES=("ReadHeavy.std" "Exchange.std" "RapidGrow.std" "ReadHeavy.ahash" "Exchange.ahash" "RapidGrow.ahash")

while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--files)
            IFS=',' read -ra DATA_FILES <<< "$2"
            shift 2
            ;;
    esac
done

set -x

cargo build --release

function plot {
    local data_file="$1"
    
    if [ ! -f "$DATA_DIR/$data_file.csv" ]; then
        echo "Warning: $DATA_DIR/$data_file.csv not found, skipping..."
        return
    fi
    
    cat "$DATA_DIR/$data_file.csv" | "$BIN" plot "$DATA_DIR" "$data_file"
}

for data_file in "${DATA_FILES[@]}"; do
    plot "$data_file"
done