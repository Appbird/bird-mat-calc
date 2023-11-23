use std::str::FromStr;

use fraction::Fraction;
fn main() {
    let mut input = String::new();
    for _i in 0..1 {
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");
        let number_in_strings:Vec<Fraction> =
            input.split_whitespace()
            .map(|x| Fraction::from_str(x).expect("Failed to parse as number"))
            .collect();
        for n in number_in_strings{
            print!("{} ", n);
        }
        println!();
    }
    
}

