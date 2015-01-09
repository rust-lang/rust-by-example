#!/bin/bash

ace_repository='https://github.com/ajaxorg/ace-builds/trunk/src-min-noconflict'
ace_local_folder='node_modules/gitbook-plugin-rust-playpen/book/ace'

mkdir -p bin
mkdir -p stage/node_modules
svn checkout ${ace_repository} ${ace_local_folder}

ln -sf ../book.json stage
ln -sf ../examples/README.md stage
