#!/bin/bash

for example in $(find examples -type d -name "*"); do
  html=stage/_book/${example#examples/}.html
  if [[ -f ${html} ]]; then
    echo ${html}

    if [[ `uname` == "Darwin" ]]; then
      sed -i '' s:${example#examples/}.md:${example}/input.md: ${html}
    else
      sed -i s:${example#examples/}.md:${example}/input.md: ${html}
    fi
  fi
done
