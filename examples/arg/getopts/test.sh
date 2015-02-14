#!/bin/sh
# Generate the example output for input.md.

function run {
    echo '$' $1
    $1
}

run './testopt'
run './testopt -a -b'
run './testopt -ab'
run './testopt -c'
run './testopt -c value'
run './testopt -c=value'
run './testopt -cvalue'
run './testopt arg'
run './testopt -a arg'
run './testopt -c value arg'
run './testopt -a -- -b' 
run './testopt -a -'
