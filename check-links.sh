#!/bin/bash

EXIT_CODE=0
UPSTREAM_BUG=("stage/_book/generics/phantom/testcase_units.html")
WHITE_LIST=("stage/_book/cast/alias.html")

WHITE_LIST+=("${UPSTREAM_BUG[@]}")

function join_by { local IFS="$1"; shift; echo "$*"; }

grep -qR ': /' ./examples
if [ "$?" -eq 0 ]; then
    echo "Some links are absolute.."
    grep -R ': /' ./examples
    EXIT_CODE=1
fi

htmlproofer stage/_book --log-level error --only-4xx --check-html --allow-hash-href --file-ignore $(join_by , "${WHITE_LIST[@]}")
if [ "$?" -ne 0 ]; then
    echo "Some links are broken.."
    EXIT_CODE=1
fi

exit $EXIT_CODE
