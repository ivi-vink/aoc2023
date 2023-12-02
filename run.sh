#!/bin/sh
day=$1
token=$2
inputdir="input/$day"
mkdir -p $inputdir
if ! [ -f "$inputdir/in.txt" ]; then
    curl \
        -s \
        -o input/$day/in.txt \
        --cookie "session=$token" \
        -- \
        https://adventofcode.com/2023/day/$day/input
else
    echo "already got input!"
fi
cargo run --bin main
