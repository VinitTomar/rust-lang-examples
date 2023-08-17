#!/usr/bin/zsh -e

filepath=$1;
 
# Extract the filename with extension from the file path
filename=$(basename "$filepath")
 
# Extract the filename without the extension
filename="${filename%.*}"

rustc $filepath --out-dir bin;
set -e
"./bin/$filename";
