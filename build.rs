extern crate cc;

fn main() {
    cc::Build::new()
        .file("c_src/directionals.c")
        .file("c_src/internals.c")
        .file("c_src/maze_gen.c")
        .include("c_src")
        .compile("maze_gen");
}
