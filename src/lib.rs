mod math;
use math::Float2;

mod tests;

fn main() {
    let mut a = Float2::new(20.0, 1.0);
    let b = a;
    a += b;
    println!("{}", a);
}