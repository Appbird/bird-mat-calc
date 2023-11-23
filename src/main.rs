mod matrix;
use matrix::Matrix;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {}

fn main() {
    let b = Matrix::read_matrix();
    let sweeped = b.sweeped();
    
    println!("{}", sweeped);
}

