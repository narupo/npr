use std::path::Path;
use std::fs;
use std::io;
use filetime::{ set_file_mtime, FileTime };

pub fn is_exists(fname: &str) -> bool {
    Path::new(fname).exists()
}

pub fn is_file(fname: &str) -> bool {
    Path::new(fname).is_file()
}

pub fn is_dir(fname: &str) -> bool {
    Path::new(fname).is_dir()
}

pub fn listdir(fname: &str) -> Result<Vec<String>, io::Error> {
    let mut v = vec![];
    let files = fs::read_dir(fname)?;

    for file in files {
        v.push(file.unwrap().path().file_name().unwrap()
            .to_os_string().into_string().unwrap());
    }

    Ok(v)
}

pub fn touch(fname: &str) -> Result<bool, io::Error> {
    if !is_exists(fname) {
        fs::File::create(fname)?;
        return Ok(false);
    }

    set_file_mtime(fname, FileTime::now())?;

    Ok(true)
}

pub fn rm(fname: &str) -> io::Result<()> {
    fs::remove_file(fname)?;
    Ok(())
}

pub fn rmdir(fname: &str) -> io::Result<()> {
    fs::remove_dir(fname)?;
    Ok(())
}

pub fn rmdirq(fname: &str) -> io::Result<()> {
    if !is_exists(fname) {
        return Ok(());
    }

    rmdir(fname)?;
    Ok(())
}

pub fn rmdirp(fname: &str) -> io::Result<()> {
    fs::remove_dir_all(fname)?;
    Ok(())
}

pub fn rmdirpq(fname: &str) -> io::Result<()> {
    if !is_exists(fname) {
        return Ok(());
    }
    
    rmdirp(fname)?;
    Ok(())
}

pub fn mkdir(fname: &str) -> io::Result<()> {
    fs::create_dir(fname)?;
    Ok(())
}

pub fn mkdirq(fname: &str) -> io::Result<()> {
    if is_exists(fname) {
        return Ok(());
    }

    mkdir(fname)?;
    Ok(())
}

pub fn mkdirp(fname: &str) -> io::Result<()> {
    fs::create_dir_all(fname)?;
    Ok(())
}

pub fn mkdirpq(fname: &str) -> io::Result<()> {
    if is_exists(fname) {
        return Ok(());
    }

    mkdirp(fname)?;
    Ok(())
}
