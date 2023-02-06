#![feature(assert_matches)]
mod funscript;
mod video;

use funscript::*;
use video::*;

fn main() {
    let path = "./test-scripts/joyfunscripter.funscript";
    let s: FScript = load_funscript(path).expect("failed to load script");
    print_script(&s);

    let sample_count = get_video_sample_count(path).expect("failed to get sample count");
}
