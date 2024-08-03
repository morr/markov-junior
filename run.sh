#!/bin/bash

# Default output file
OUTPUT_FILE="output.txt"
REFRESH_INTERVAL=1  # Default refresh interval in seconds

# Function to show usage
usage() {
    echo "Usage: $0 [--output OUTPUT_FILE] [--refresh-interval SECONDS] [OTHER_RUST_PROGRAM_ARGS...]"
    exit 1
}

# Parse arguments
RUST_ARGS=()
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --refresh-interval)
            REFRESH_INTERVAL="$2"
            shift 2
            ;;
        --help)
            usage
            ;;
        *)
            RUST_ARGS+=("$1")
            shift
            ;;
    esac
done

# Ensure the output file argument is always passed
RUST_ARGS+=("--output" "$OUTPUT_FILE")

# Function to clean up background processes on script exit
cleanup() {
    # echo "Cleaning up..."
    pkill -P $RUST_PID  # Kill the Rust program
    pkill -P $$  # Kill all child processes of this script
    exit 0
}

# Trap SIGINT and SIGTERM signals to run the cleanup function
trap cleanup SIGINT SIGTERM

# Run the Rust program
cargo build --release && cargo run --release -- "${RUST_ARGS[@]}" &
RUST_PID=$!

# Use entr to watch the output file and display updates with delay
while true; do
    echo "$OUTPUT_FILE" | entr -d sh -c "cat $OUTPUT_FILE | pattern-to-png 1x | imgcat --width=50; sleep $REFRESH_INTERVAL"
done &

# Wait for the Rust program to finish
wait $RUST_PID

# Cleanup after Rust program finishes
cleanup
