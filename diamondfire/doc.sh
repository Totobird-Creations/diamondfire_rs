set -e

printf "\x1b[32mDocumenting Library...\x1b[0m\n"
RUSTFLAGS="--cfg diamondfire_doc" RUSTDOCFLAGS="--cfg diamondfire_doc" cargo doc --target=x86_64-unknown-linux-gnu
