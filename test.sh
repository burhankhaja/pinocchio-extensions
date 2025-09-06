# 1. build and test with "show-output":     ./test.sh s
# 2. build and test:                        ./test.sh

if [[ "$1" == "s" ]]; then
    ./build.sh && (cd tests && clear && cargo test -- --show-output)
else
    ./build.sh && (cd tests && clear && cargo test)
fi
