const PHF_FILENAME: &str = "phf.rs";

#[path = "src/block_starts.rs"]
mod block_starts;

fn main() {
    use phf_codegen::Map;
    use std::{env::var_os, fs::File, path::PathBuf};

    let output_dir = PathBuf::from(var_os("OUT_DIR").unwrap());
    let mut output_source = File::create(output_dir.join(PHF_FILENAME)).unwrap();

    let mut phf = Map::new();
    for i in 0..block_starts::BLOCK_STARTS.len() {
        phf.entry(block_starts::BLOCK_STARTS[i], &i.to_string()[..]);
    }
    phf.build(&mut output_source).unwrap();
}
