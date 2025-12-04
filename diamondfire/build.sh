cat ../diamondfire-unknown-unknown.json.template \
    | sed -e "s|{DIAMONDFIRERS_ROOT}|$(dirname ${PWD})|g" \
    > ../diamondfire-unknown-unknown.json

rm rustc-ice-*.txt
rm ../diamondfire-macros/rustc-ice-*.txt
printf "\n\x1b[32mCompiling Codegen Library...\x1b[0m\n"
(cd ../rustc_codegen_diamondfire; cargo build || exit 1)
printf "\n\x1b[32mCompiling Linker Executable...\x1b[0m\n"
(cd ../linker_diamondfire; cargo build || exit 1)
printf "\n\x1b[32mCompiling Example Library...\x1b[0m\n"
cargo build -Zbuild-std --target=../diamondfire-unknown-unknown.json --example=transform || exit 1
