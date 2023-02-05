#![feature(assert_matches)]
mod funscript;

use funscript::*;

fn main() {
    let path = "./test-scripts/joyfunscripter.funscript";
    let s: FScript = load_funscript(path).expect("failed to load script");
    print_script(&s);
}
