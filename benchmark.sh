#!/bin/bash 

set -Cue

for directory in ./*/; do
    TARGET=$(basename $directory)
    echo "Benchmarking $TARGET..."
    cd $TARGET
    mkdir -p .oha || :
    cargo run --release &
    sleep 30
    for CONCURRENCY in 16 32 64 128 256 512; do
        echo "[concurrency: $CONCURRENCY]"
        sleep 30
        oha \
            --wait-ongoing-requests-after-deadline \
            --no-tui \
            --disable-keepalive \
            --latency-correction \
            -c $CONCURRENCY \
            -z 15s \
            --output ".oha/C_$CONCURRENCY.log" \
            'http://localhost:3000/user/1234567890987654321'
    done
    pkill -f "target/release/$TARGET"
    cd ..
done
