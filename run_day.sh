# Ussage: ./run_day.sh [day]
# Runs all rust projects with [day] in the name

# Run all projects with [day] in the name
# Search only top level directories

for i in $(find . -maxdepth 1 -type d -name "day$1*"); do
    echo "Running $i"
    cd $i
    cargo run
    cd ..
done