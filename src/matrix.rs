use std::str::FromStr;
use fraction::{Fraction, Zero};

type MatrixArray = Vec::<Vec::<Fraction>>;
#[derive(Clone)]
pub struct Matrix{
    pub mat: MatrixArray,
    row: usize,
    col: usize,
}

impl Matrix {
    pub fn read_matrix() -> Matrix {
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
    pub fn push_back_id(self) -> Matrix {
        let mut matrix = Matrix::zero(self.row + self.col, self.col);

        return Matrix::as_matrix(matrix);
    }

    
    pub fn zero(row:usize, col:usize) -> Self{
        Matrix {
            mat: vec![vec![Fraction::from(0); col]; row],
            row, col
        }
    }
    pub fn as_matrix(mat : Vec::<Vec::<Fraction>>) -> Self{
        assert!(mat.len() > 0);
        for row in mat.iter().skip(1) {
            assert_eq!(row.len(), mat[0].len());
        }
        let row = mat.len();
        let col = mat[0].len();
        Matrix { mat, row, col }
    }
    pub fn row(&self) -> usize{
        self.row
    }
    pub fn col(&self) -> usize{
        self.col
    }
    pub fn sweeped(&self) -> Matrix {
        let mut sweeped = self.clone();
        for r in 0..self.row() {
            // アンカーが0になることを防ぐべく、0でない行列を探す。
            {
                let mut r_1 = r;
                while sweeped.mat[r_1][r] == Fraction::zero() {
                    r_1 += 1;
                    if r_1 == sweeped.row() { return sweeped; }
                }
                sweeped.mat.swap(r_1, r);
            }
            let anchor = sweeped.mat[r][r];

            for l in r..self.col() {
                sweeped.mat[r][l] /= anchor;
            }

            for k in 0..self.row() {
                if r == k { continue; }
                let current_top = sweeped.mat[k][r];
                for l in r..self.col() {
                    sweeped.mat[k][l] = sweeped.mat[k][l] - sweeped.mat[r][l] * current_top;
                }
            }
            println!("{}", sweeped);
        }
        return sweeped;
    }

    fn inversed(&self) -> Matrix{
        
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

