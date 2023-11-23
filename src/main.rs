mod matrix;
use clap::{Parser, Subcommand};
use crate::matrix::Matrix;

#[derive(Subcommand)]
enum Commands {
    /// calculate product of two given matrix through stdin.
    Mul,
    /// Sweeping one given matrix through stdin.
    Sweep,
    /// inverse one given matrix through stdin.
    Inverse,

}

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct CLI {
    #[command(subcommand)]
    /// 空白区切りで入力を行い、改行を行えば次の列へ入力される。
    /// 空行を含めば、次の行列へ入力が移る。
    command: Commands
}
fn eprint_guide() {
    eprintln!("To finish entering one matrix, enter an empty line.");
}

fn mul() {
    eprintln!("Enter A >>");
    let a = Matrix::read_matrix();
    
    eprintln!("Enter B (its row should be {}.) >>", a.col());
    let b = Matrix::read_matrix();

    println!("{}", a.mul(&b));
}
fn sweep() {
    eprintln!("Enter A >>");
    let a = Matrix::read_matrix();

    println!("{}", a.sweeped());
}
fn inverse() {
    eprintln!("inverse A. Finish entering a matrix by enter empty line.");
    eprintln!("Enter A >>");
    let a = Matrix::read_matrix();

    println!("{}", a.inversed());
}

fn main() {
    env_logger::init();
    let cli = CLI::parse();
    eprint_guide();
    match  &cli.command {
        Commands::Mul => mul(),
        Commands::Sweep => sweep(),
        Commands::Inverse => inverse(),
    }
    
}

