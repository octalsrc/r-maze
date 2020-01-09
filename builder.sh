nix-build && LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(cat result) nix-shell -p stdenv --command "cargo run"
