use std::str::FromStr;

use fraction::Fraction;
fn main() {
    let mut matrix = Vec::<Vec::<Fraction>>::new();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");
        if input.trim().len() == 0 { break; }

        let number_in_strings:Vec<Fraction> =
            input.split_whitespace()
            .map(|x| Fraction::from_str(x).expect("Failed to parse as number"))
            .collect();
        matrix.push(number_in_strings);

    }
    for row in matrix {
        for cell in row {
            print!("{} ", cell);   
        }
        println!();
    }
    
}

