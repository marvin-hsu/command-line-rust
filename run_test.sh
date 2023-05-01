for dir in [0-9][0-9]_*; do
    if [ -d "$dir" ]; then
        echo "Running tests for $dir"
        (cd "$dir" && cargo test)
    fi
done
