#!/bin/bash

WHITELIST=(
  ./examples/lifetime/borrow/borrow.rs
  ./src/playpen.rs
)

echo "Checking if any rust file has a line longer than 99 characters"

suspects=$(find . -name '*.rs' | xargs grep -El ".{100}")
status=$?

any_offender=false
if [[ $status == 0 ]]; then
  for suspect in $suspects; do
    if [[ " ${WHITELIST[*]} " == *" ${suspect} "* ]]; then
      continue
    fi
    any_offender=true
    echo "> $suspect exceeds 79 chars"
    awk 'length($0) > 79' $suspect
  done

fi

if $any_offender; then
  exit 1
else
  echo "All is good!"
fi
