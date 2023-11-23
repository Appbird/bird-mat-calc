use fraction;

type F = fraction::Fraction;
fn main() {
    let two = F::from(0) + 2;
    let two_third = two / 3;
    println!("{}", two_third);
}
