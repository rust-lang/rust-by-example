#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd book

git init
git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"
git remote add upstream "https://$GITHUB_TOKEN@github.com/rust-lang/rust-by-example.git"
git fetch upstream && git reset upstream/gh-pages

echo "rustbyexample.com" > CNAME

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
