#!/bin/bash

for example in $(ls examples); do
  if [[ -d examples/${example} ]]; then
    html=stage/_book/${example}.html

    sed -i s:${example}.md:examples/${example}/input.md: ${html}
  fi
done
