#!/bin/bash
# creates two new rust projects for the day
# Usage: ./init_day.sh <day> <part> (e.g. ./init_day.sh 1 a)
# part is optional and defaults to a
part=${2:-a}

# check if day is given
if [ -z "$1" ]; then
    echo "No day given"
    exit 1
fi

# make sure day has 2 digits
day=$(printf "%02d" $1)

project="day$day$part"

if [ -d $project ]; then
    echo "Project $project already exists"
    exit 1
fi

project_a="day${day}a"
echo "Creating project $project"

# if part is a, create project a
if [ $part = "a" ]; then
    cargo new $project
    # add input.txt
    touch $project/input.txt
    # replace main.rs with template.rs
    cp template.rs $project/src/main.rs
else
    # check if project a exists
    if [ ! -d $project_a ]; then
        echo "Project $project_a does not exist"
        exit 1
    fi

    cargo new $project
    # add input.txt from project a
    cp $project_a/input.txt $project/input.txt
    # copy main.rs from project a
    cp $project_a/src/main.rs $project/src/main.rs
fi
