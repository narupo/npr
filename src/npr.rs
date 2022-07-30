use std::path::Path;
use std::fs;
use std::io;

pub fn exists(fname: &str) -> bool {
    Path::new(fname).exists()
}

pub fn listdir(fname: &str) -> Result<Vec<String>, io::Error> {
    let mut v = vec![];
    let files = fs::read_dir(fname);

    let files = match files {
        Ok(files) => files,
        Err(err) => return Err(err)
    };

    for file in files {
        v.push(file.unwrap().path().into_os_string().into_string().unwrap());
    }

    Ok(v)
}
