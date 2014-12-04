#!/bin/bash

repo="https://$GH_TOKEN@github.com/japaric/rust-by-example.git"

git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"

rev=$(git rev-parse --short HEAD)

cd stage/_book

git init -q
git remote add up ../../.git
git fetch -q up && git reset -q --hard up/gh-pages

touch .

git add .

git commit -m "rebuild pages at ${rev}"
git push -q o HEAD:gh-pages
