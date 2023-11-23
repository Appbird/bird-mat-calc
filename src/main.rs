mod matrix;
use matrix::Matrix;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// 二つの行列の積をとる。
    /// 空白区切りで入力を行い、改行を行えば次の列へ入力される。
    /// 空行を含めば、次の行列へ入力が移る。
    Mul,
    /// 行列に対して掃き出し法を行う。
    Sweep,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands
}

fn mul() {
    let a = Matrix::read_matrix();
    let b = Matrix::read_matrix();
}

fn main() {
    let cli = CLI::parse();
    let b = Matrix::read_matrix();
    match  &cli.command {
        Commands::Mul => ,
        Commands::Sweep =>
    }
    
}

