mod npr;
use npr as r;

fn test_is_exists() {
    assert_eq!(r::is_exists("LICENSE"), true);
}

fn test_listdir() {
    let files = r::listdir("sample/listdir");
    let files = files.unwrap();
    assert_eq!(files.len(), 3);
    assert_eq!(files[0], "f1.txt");
    assert_eq!(files[1], "f2.txt");
    assert_eq!(files[2], "f3.txt");
}

fn test_touch() {
    let fname = "sample/touch/f.txt";
    r::rm(fname).expect("failed to rm touch/f.txt");

    let result = r::touch(fname).unwrap();
    assert_eq!(result, false);

    let result = r::touch(fname).unwrap();
    assert_eq!(result, true);
}

fn test_is_file() {
    assert_eq!(r::is_file("LICENSE"), true);
}

fn test_is_dir() {
    assert_eq!(r::is_dir("sample"), true);
}

fn test_mkdir() {
    let fname = "sample/mkdir/";
    if !r::is_exists(fname) {
        r::mkdir(fname).unwrap();
    }
    let fname = "sample/mkdir/dir";
    if r::is_exists(fname) {
        r::rmdir(fname).unwrap();
    }
    r::mkdir(fname).unwrap();
    assert_eq!(r::is_exists(fname), true);
}

fn test_mkdirq() {
    let fname = "sample/mkdir/";
    r::mkdirq(fname).unwrap();

    let fname = "sample/mkdir/dir";
    if r::is_exists(fname) {
        r::rmdir(fname).unwrap();
    }
    r::mkdirq(fname).unwrap();
    assert_eq!(r::is_exists(fname), true);
}

fn test_mkdirp() {
    let fname = "sample/mkdir/";
    if !r::is_exists(fname) {
        r::mkdir(fname).unwrap();
    }

    let fname1 = "sample/mkdir/dirp/";
    if r::is_exists(fname1) {
        r::rmdirp(fname1).unwrap();
    }
    let fname2 = "sample/mkdir/dirp/";

    let fname3 = "sample/mkdir/dirp/dirp/";
    r::mkdirp(fname3).unwrap();
    assert!(r::is_exists(fname1));
    assert!(r::is_exists(fname2));
}

fn test_mkdirpq() {
    let fname = "sample/mkdir/";
    if !r::is_exists(fname) {
        r::mkdir(fname).unwrap();
    }

    let fname1 = "sample/mkdir/dirp";
    let fname2 = "sample/mkdir/dirp/dirp/";
    r::mkdirpq(fname2).unwrap();
    assert!(r::is_exists(fname1));
    assert!(r::is_exists(fname2));
}

fn test_rmdir() {
    let fname = "sample/rmdir";
    r::mkdirq(fname).unwrap();

    r::rmdir(fname).unwrap();
    assert!(!r::is_exists(fname));
}

fn test_rmdirq() {
    let fname = "sample/rmdir";
    r::rmdirq(fname).unwrap();
    assert!(!r::is_exists(fname));
}

fn test_rmdirp() {
    let fname = "sample/rmdirp/dir";
    r::mkdirpq(fname).unwrap();

    r::rmdirp(fname).unwrap();
    assert!(!r::is_exists(fname));
}

fn test_rmdirpq() {
    let fname = "sample/rmdirp/dir";
    r::rmdirpq(fname).unwrap();
    assert!(!r::is_exists(fname));
}

fn main() {
    test_is_exists();
    test_is_file();
    test_is_dir();
    test_listdir();
    test_touch();
    test_mkdir();
    test_mkdirq();
    test_mkdirp();
    test_mkdirpq();
    test_rmdir();
    test_rmdirq();
    test_rmdirp();
    test_rmdirpq();
}
