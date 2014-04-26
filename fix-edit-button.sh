#!/bin/bash

for example in $(ls output/examples); do
  html=output/_book/examples/${example}/README.html

  sed -i s:examples/${example}/README.md:src/${example}/input.md: ${html}
done
