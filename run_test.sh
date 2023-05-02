cargo install cargo2junit
for dir in [0-9][0-9]_*; do
    if [ -d "$dir" ]; then
        echo "Running tests for $dir"
        (cd "$dir" && cargo test -- --format json | cargo2junit > results.xml)
    fi
done

cat results*.xml > combined_results.xml

echo "::set-output name=combined_results_file::combined_results.xml"
echo "::set-output name=combined_results_path::$(realpath combined_results.xml)"
