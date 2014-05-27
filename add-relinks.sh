#!/bin/bash

for example in $(ls examples); do
  if [[ -d examples/${example} ]]; then
    new_html=www.rustbyexample.com/examples/${example}/README.html

    mkdir -p stage/_book/examples/${example}
    echo '<script type="text/javascript">
window.location = "'${new_html}'"
</script>' > stage/_book/examples/${example}/README.html
  fi
done
