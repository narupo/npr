mod npr;
use npr as r;

fn test_exists() {
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
    r::remove(fname).expect("failed to remove touch/f.txt");

    let result = r::touch(fname).unwrap();
    assert_eq!(result, false);

    let result = r::touch(fname).unwrap();
    assert_eq!(result, true);
}

fn main() {
    test_exists();
    test_listdir();
    test_touch();
}
