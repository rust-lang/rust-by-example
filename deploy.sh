#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd stage/_book

git init
git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"
git remote add upstream "https://$GH_TOKEN@github.com/rust-lang/rust-by-example.git"
git fetch upstream && git reset upstream/gh-pages

echo "rustbyexample.com" > CNAME
cp -r ../../vendor/gitbook/* gitbook/

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
