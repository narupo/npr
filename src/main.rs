mod npr;
use npr as r;

fn main() {
    assert_eq!(r::exists("LICENSE"), true);

    let files = r::listdir("sample/listdir");
    assert_eq!(files.len(), 3);
}
