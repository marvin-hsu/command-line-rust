cargo install cargo2junit
for dir in [0-9][0-9]_*; do
    if [ -d "$dir" ]; then
        echo "Running tests for $dir"
        (cd "$dir" && cargo test -- -Z unstable-options --format json --report-time | cargo2junit > results.xml)
    fi
done

find . -name 'results.xml' -type f -print0 | xargs -0 cat > combined_results.xml
