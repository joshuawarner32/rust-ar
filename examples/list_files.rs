extern crate ar;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} somefile.a", args[0]);
        return;
    }

    let mut contents = Vec::new();
    File::open(&args[1]).expect("readable file").read_to_end(&mut contents).expect("read succeeds");

    for f in ar::Reader::new(&contents).expect("valid archive header") {
        println!("{} {} {} {} {} {}", f.name().expect("valid utf8"), f.contents().len(), f.modified_timestamp(), f.owner_id(), f.group_id(), f.file_mode());
    }
}
