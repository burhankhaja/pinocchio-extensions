clear

for program in examples/*/; do
    if [ -d "$program" ]; then
        echo "Building $(basename "$program")..."
        cd "$program" && cargo build-sbf && cd - > /dev/null
    fi
done
