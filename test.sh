#!/usr/bin/zsh -e


fileName=$1;
outputfile=$(sed -e 's/\(\.rs\)*$//g' <<<$fileName)

rustc $fileName --out-dir bin;
set -e
"./bin/$outputfile";
