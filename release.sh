#!/bin/sh
cargo set-version --bump patch
cargo get version > ~/HttpFileServer/self_update_test/x86_64/version
cargo run
mv ~/HttpFileServer/self_update_test/x86_64/self_update_test ~/HttpFileServer/self_update_test/x86_64/self_update_test_`date +%Y%m%d`
cp ~/self_update_test/target/debug/self_update_test ~/HttpFileServer/self_update_test/x86_64/self_update_test

