mod npr;
use npr as r;

fn main() {
    assert_eq!(r::exists("LICENSE"), true);

    let files = r::listdir("sample/listdir");
    let files = files.unwrap();
    assert_eq!(files.len(), 3);
    assert_eq!(files[0], "f1.txt");
    assert_eq!(files[1], "f2.txt");
    assert_eq!(files[2], "f3.txt");
}
