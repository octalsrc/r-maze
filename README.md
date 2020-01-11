R-Maze
======

This is a Rust re-implementation of the previous
[`c-maze`](https://github.com/octalsrc/c-maze), which was written in
C.  Main purpose being to test out the `piston` framework.

Building and running
--------------------

If you have the Nix package manager, simply run

    $ ./play.sh

This will make the proper X libraries accessible and then perform a
`cargo run` to launch the game.

If you're not running NixOS, just try `cargo run` by itself---if the
game crashes it should tell you which X library it couldn't find and
then you hopefully can install it with your local package manager.

Playing the game
----------------

Controls: `W-A-S-D` to move, `ESC` to quit.

Your goal is to find the legendary treasure known as the *Eye of the
Pharaohs*.  This task is complicated by a maze of dark, twisty
hallways lit only by your flashlight.
