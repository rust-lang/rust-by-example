#!/bin/bash

export LANG="en_US.UTF-8"

echo "Checking if any rust file has a line longer than 75 characters"

offenders=$(grep -Pl ".{76}" src/*/*.rs)
status=$?

if [[ $status == 0 ]]; then
  for offender in $offenders; do
    echo "> $offender exceeds 75 chars"
    awk 'length($0) > 75' $offender
  done

  exit 1
fi

echo "All is good!"
