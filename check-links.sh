#!/bin/bash

# 
# Extract links from Markdown documents, and check 
# their HTTP status. Flag anything that doesn't 
# return a 200.
# 

DELIMITER="{{-%%-}}"

echo "Checking links..."

any_bad_links=false

files=$(find ./examples -name "*.md")

for file in $files ; do
  
  # 
  # Extract the urls, if any, along with the line numbers.
  # 
  reference_style_links=$(grep -n "^\[[^]]\+\]:\ http" $file | \
                          sed -e "s/:/$DELIMITER/" -e "s/\[[^]]*\]: //")
  
  inline_links=$(grep -no "\[[^]]\+\]([^[:space:]]\+)" $file | \
                 sed -e "s/:/$DELIMITER/" -e "s/\[[^]]*\](//" -e "s/)$//")
    
  if [[ $reference_style_links == "" && $inline_links == "" ]]; then
    continue
  elif [[ $reference_style_links == "" ]]; then
    all_links="$inline_links"
  elif [[ $inline_links == "" ]]; then
    all_links="$reference_style_links"
  else    
    all_links=$(echo "$reference_style_links"$'\n'"$inline_links" | sort -n)
  fi
    
  for link in $all_links ; do
    
    url=$(echo $link | awk -F"$DELIMITER" '{print $2}')
    
    # Check relative, internal, urls
    if [[ ! $url == http://* ]] && [[ ! $url == https://* ]]; then
        
        local_path=$url
        
        # Remove the .html if present
        if [[ $local_path == *.html ]]; then
            local_path=${local_path:0:${#local_path}-5}
        fi
        
        # Build the local directory path
        # This depends on the GitBook style directory structure
        if [[ ! $local_path == /* ]]; then
            local_path="./examples/$local_path"
        else
            local_path="./examples$local_path"
        fi
        
        if [[ -d $local_path ]]; then
            continue
        fi
        
        status_code="404"
        
    # Check external urls
    else
        
        # -s: silent
        # -L: follow redirect
        # -o: send output to /dev/null
        # -I: load headers only
        # -w: write the status code to stdout
        status_code=$(curl -s -L -o /dev/null -I -w "%{http_code}" "$url")
    
        if [[ $status_code == "200" ]]; then
          continue
        fi  
    fi
        
    any_bad_links=true
    
    line_number=$(echo $link | awk -F"$DELIMITER" '{print $1}')
    
    echo -e "Bad link in $file:$line_number [$status_code] $url"
  done
  
done

if $any_bad_links; then
  echo "Some links were bad."
else
  echo "All links are 200!"
fi
