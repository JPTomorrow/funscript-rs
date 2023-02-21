#![feature(assert_matches)]
mod funscript;

use crate::funscript::*;
// use native_dialog::FileDialog;

fn main() {
    // let path = FileDialog::new()
    //     .set_location("~/")
    //     .add_filter("Funscript Files", &["funscript"])
    //     .show_open_single_file()
    //     .unwrap();

    // let path = match path {
    //     Some(path) => path,
    //     None => panic!("no file selected!"),
    // };

    // let path_str = path.to_str().unwrap();

    // let mut s: FScript = load_funscript(path_str).expect("failed to load script");
    // print_script_diagnostics(&s);
    // apply_rdp(&mut s, 10.0);
    // print_script_diagnostics(&s);
    // save_funscript(path_str, &s).expect("failed to save script");
}
