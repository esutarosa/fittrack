#!/usr/bin/env sh

set -eu

source_file=".env.example"
target_file=".env"

if [ ! -f "$source_file" ]; then
  echo "Missing $source_file"
  exit 1
fi

if [ -f "$target_file" ]; then
  echo "$target_file already exists"
  exit 0
fi

cp "$source_file" "$target_file"
echo "Created $target_file from $source_file"
