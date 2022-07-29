mod npr;
use npr as r;

fn main() {
    assert_eq!(r::exists("LICENSE"), true);
}
