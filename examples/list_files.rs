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
        match f.name() {
            Some(name) => println!("{}", name),
            None => println!("{:?}", f.name_u8())
        }
    }
}
