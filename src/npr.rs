use std::path::{ Path };
use std::fs;

pub fn exists(fname: &str) -> bool {
    Path::new(fname).exists()
}

pub fn listdir(fname: &str) -> Vec<String> {
    let mut v = vec![];

    for file in fs::read_dir(fname).unwrap() {
        v.push(file.unwrap().path().into_os_string().into_string().unwrap());
    }

    v
}
