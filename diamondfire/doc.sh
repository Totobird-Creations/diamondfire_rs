rm rustc-ice-*.txt
rm ../diamondfire-macros/rustc-ice-*.txt
printf "\n\x1b[32mCompiling Codegen Library...\x1b[0m\n"
(cd ../rustc_codegen_diamondfire; cargo build || exit 1)
printf "\n\x1b[32mCompiling Linker Executable...\x1b[0m\n"
(cd ../diamondfire-linker; cargo build || exit 1)
printf "\n\x1b[32mCompiling Example Library...\x1b[0m\n"
RUSTFLAGS="--cfg diamondfiresys_docsrs" cargo doc || exit 1
