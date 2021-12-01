#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "The day number must be provided!	"
fi

mkdir -p src/bin

# Create the source from the template
DAY_NUMBER=$1
TEMPLATE_PATH="template/dayXX.in"
OUTPUT_PATH="src/bin/day$DAY_NUMBER.rs"

sed "s/%daynumber%/$DAY_NUMBER/g" $TEMPLATE_PATH > $OUTPUT_PATH

# Add also to the Cargo.toml file
echo "" >> Cargo.toml
echo "[[bin]]" >> Cargo.toml
echo "name = \"day$DAY_NUMBER\"" >> Cargo.toml


