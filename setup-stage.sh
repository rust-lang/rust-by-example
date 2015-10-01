#!/bin/bash

ace_repository='https://github.com/ajaxorg/ace-builds/trunk/src-min'
ace_playpen_local='node_modules/gitbook-plugin-rust-playpen'

mkdir -p bin
mkdir -p stage/node_modules
svn checkout ${ace_repository} ${ace_playpen_local}'/book/ace'
cp ${ace_playpen_local}'/book/ace/mode-rust.js' ${ace_playpen_local}'/book/mode-rust.js'

ln -sf ../book.json stage
ln -sf ../examples/README.md stage
