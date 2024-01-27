#!/bin/zsh

# Requires ghp-import to be installed (e.g. via pip3)

rm -rf target/doc &&  # purge old docs that may include docs for deps
cargo doc --no-deps --features=llvm-17 &&  # document just this crate
echo "<meta http-equiv=refresh content=0;url=llvm_ir/index.html>" > target/doc/index.html &&  # put in the top-level redirect
ghp-import -np target/doc &&  # publish to gh-pages branch
rm -rf target/doc &&  # kill the docs that were just this crate
cargo doc --features=llvm-17  # regenerate all docs (including deps) for local use
