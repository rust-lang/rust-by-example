#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd stage/_book

git init
git config user.name "Igor.Shaposhnik"
git config user.email "shaposhnikigor95@bk.ru"
git remote add upstream "https://$GH_TOKEN@github.com/ruRust/rust-by-example-ru.git"
git fetch upstream && git reset upstream/gh-pages

#echo "rustbyexample.com" > CNAME
cp -r ../../vendor/gitbook/* gitbook/

#fix editor syntax highlight
sed -i 's#/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js#/rust-by-example-ru/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js#g' gitbook/plugins/gitbook-plugin-rust-playpen/editor.js

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
