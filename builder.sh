nix-build && LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(cat result) cargo run
