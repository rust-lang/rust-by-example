#!/bin/bash

echo "Checking if any markdown file has a line longer than 99 characters"

suspects=$(find ./examples/ -name '*.md' | xargs grep -El ".{100}")
status=$?

any_offender=false
if [[ $status == 0 ]]; then
  for suspect in $suspects; do
    any_offender=true
    echo "> $suspect exceeds 99 chars"
  done

fi

if $any_offender; then
  exit 1
else
  echo "All is good!"
fi
