#!/bin/bash

for example in $(find examples -type d -name "*"); do
  html=stage/_book/${example#examples/}.html
  if [[ -f ${html} ]]; then
    echo ${html}

    sed -i -e s:${example#examples/}.md:${example}/input.md: ${html}
  fi
done
