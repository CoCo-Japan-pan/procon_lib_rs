#!/bin/bash 

# the directory of this script
DIR="$(dirname "$BASH_SOURCE")"

bundle_libs_atcoder() {
    cargo equip --remove docs --minify libs --exclude-atcoder-202301-crates --mine github.com/CoCo-Japan-pan | xsel -b
}

bundle_libs() {
    cargo equip --remove docs --minify libs --mine github.com/CoCo-Japan-pan | xsel -b
}

gen_template() {
    cargo generate --path $DIR/template
}