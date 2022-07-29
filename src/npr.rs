use std::path::{ Path };

pub fn exists(fname: &str) -> bool {
    Path::new(fname).exists()
}
