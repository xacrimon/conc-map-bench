#!/usr/bin/env bash

BIN=./target/release/conc-map-bench
OUT=./results

WORKLOADS=("ReadHeavy" "Exchange" "RapidGrow")
HASHERS=("std" "ahash")

EXTRA_ARGS=()
while [[ $# -gt 0 ]]; do
    case $1 in
        -w|--workload)
            if [[ "$2" == "Custom" ]]; then
                WORKLOADS=("Custom")
            else
                IFS=',' read -ra WORKLOADS <<< "$2"
            fi
            shift 2
            ;;
        -h|--hasher)
            IFS=',' read -ra HASHERS <<< "$2"
            shift 2
            ;;
        *)
            # Pass all other arguments through
            EXTRA_ARGS+=("$1")
            shift
            ;;
    esac
done

set -x

cargo build --release
mkdir -p "$OUT"

function bench {
    local workload="$1"
    local hasher="$2"
    
    date

    file="$OUT/$workload.$hasher.csv"

    if [ -s "$file" ]; then
        EXTRA_ARGS+=("--csv-no-headers")
    fi

    if ! "$BIN" bench -w "$workload" -h "$hasher" "${EXTRA_ARGS[@]}" --csv 2>>"$file"; then
        bench "$workload" "$hasher"
    fi
}

for workload in "${WORKLOADS[@]}"; do
    for hasher in "${HASHERS[@]}"; do
        bench "$workload" "$hasher"
    done
done

date
