bundle_libs_atcoder() {
    cargo equip --remove docs --minify libs --exclude-atcoder-202301-crates --mine github.com/CoCo-Japan-pan | xsel -b
}

bundle_libs() {
    cargo equip --remove docs --minify libs --mine github.com/CoCo-Japan-pan | xsel -b
}