# 1. build and test with "show-output":     ./test.sh s
# 2. build and test:                        ./test.sh

(
    clear && cd ..

    for program in examples/*/; do
        if [ -d "$program" ]; then
            echo "Building $(basename "$program")..."
            cd "$program" && cargo build-sbf && cd - > /dev/null
        fi
    done
)

if [[ "$1" == "s" ]]; then
    (clear && cargo test -- --show-output)
else
    (clear && cargo test)
fi
