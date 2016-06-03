extern crate ar;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} somefile.a", args[0]);
        return;
    }

    let mut contents = Vec::new();
    File::open(&args[1]).expect("readable file").read_to_end(&mut contents).expect("read succeeds");

    for f in ar::Reader::new(&contents).expect("valid archive header") {
        if f.file_mode() != 0 {
            let path = Path::new(f.name().expect("valid utf8"));
            let mut out = File::create(path).expect("create succeeds");
            out.write_all(f.contents()).expect("write succeeds");
        }
    }
}
