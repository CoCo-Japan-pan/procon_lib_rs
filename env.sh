#!/bin/bash 

bundle_libs_atcoder() {
    cargo equip --remove docs --minify libs --exclude-atcoder-202301-crates --mine github.com/CoCo-Japan-pan | xsel -b
}

bundle_libs() {
    cargo equip --remove docs --minify libs --mine github.com/CoCo-Japan-pan | xsel -b
}

# Generate a template lib crate by cargo-generate
gen_template() {
    cargo generate --git https://github.com/CoCo-Japan-pan/template-for-procon-lib-rs.git
}