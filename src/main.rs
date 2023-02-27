use tiny_skia::*;

fn main() {
    let mut a = vec![1, 2, 3];
    println!("{:?}", a);
    let p = &mut a[..];
    p[0] = 2;
    println!("{:?}", a);
}
