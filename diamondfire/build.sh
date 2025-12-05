# Move to the directory where this file is.
# Makes sure we don't accidentally remove files from other directories.
cd $(dirname ${0})

# Remove files from previous compilations.
# This ensures that all artifacts are regenerated.
# This is unneeded unless rustc_codegen_diamondfire has been modified.
rm rustc-ice-*.txt
rm -rd target
rm ../diamondfire-macros/rustc-ice-*.txt
rm ../diamondfire-sys/rustc-ice-*.txt

# Fills out the target json template with the correct absolute paths.
cat ../diamondfire-unknown-unknown.json.template \
    | sed -e "s|{DIAMONDFIRERS_ROOT}|$(dirname ${PWD})|g" \
    > ../diamondfire-unknown-unknown.json

# Builds the codegen library and linker executable.
printf "\n\x1b[32mCompiling Codegen Library...\x1b[0m\n"
(cd ../rustc_codegen_diamondfire; cargo build || exit 1)
printf "\n\x1b[32mCompiling Linker Executable...\x1b[0m\n"
(cd ../linker_diamondfire; cargo build || exit 1)

# Builds the example library.
printf "\n\x1b[32mCompiling Example Library...\x1b[0m\n"
cargo build -Zbuild-std --target=../diamondfire-unknown-unknown.json --example=static --release || exit 1
