use clap::Parser;
use std::str::FromStr;
use fraction::Fraction;

type MatrixArray = Vec::<Vec::<Fraction>>;
struct Matrix{
    pub mat: MatrixArray,
    row: usize,
    col: usize,
}

impl Matrix {
    fn zero(row:usize, col:usize) -> Self{
        Matrix {
            mat: vec![vec![Fraction::from(0); col]; row],
            row, col
        }
    }
    fn as_matrix(mat : Vec::<Vec::<Fraction>>) -> Self{
        assert!(mat.len() > 0);
        for row in mat.iter().skip(1) {
            assert_eq!(row.len(), mat[0].len());
        }
        let row = mat.len();
        let col = mat[0].len();
        Matrix { mat, row, col }
    }
    fn row(&self) -> usize{
        self.row
    }
    fn col(&self) -> usize{
        self.col
    }
}

impl std::ops::Mul for Matrix{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.col(), rhs.row());
        let mut c = Matrix::zero(self.row(), rhs.col());
        for i in 0..self.row() {
            for j in 0..rhs.col(){
                for k in 0..self.col() {
                    c.mat[i][j] += self.mat[i][k] * rhs.mat[k][j];
                }
            }
        }
        return c;
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.mat {
            for cell in row {
                write!(f, "{} ", cell).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

fn read_matrix() -> Matrix {
    let mut matrix = MatrixArray::new();
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
    return Matrix::as_matrix(matrix);
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {}

fn main() {
    let a = read_matrix();
    let b = read_matrix();
    let c = a * b;
    
    println!("{}", c);
}

