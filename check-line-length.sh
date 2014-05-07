#!/bin/bash

WHITELIST=(
  src/lifetime/lifetime.rs
)

echo "Checking if any rust file has a line longer than 75 characters"

suspects=$(grep -Pl ".{76}" src/*/*.rs)
status=$?

any_offender=false
if [[ $status == 0 ]]; then
  for suspect in $suspects; do
    if [[ $WHITELIST == *${suspect}* ]]; then
      continue
    fi
    any_offender=true
    echo "> $suspect exceeds 75 chars"
    awk 'length($0) > 75' $suspect
  done

fi

if $any_offender; then
  exit 1
else
  echo "All is good!"
fi
