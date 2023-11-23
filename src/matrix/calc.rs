use std::{process::exit, cmp::min};
use super::basis::Matrix;
use fraction::{Fraction, Zero};
use log::error;

pub enum Regularity {
    Regular(Matrix),
    Sigular(Matrix),
}

impl Matrix {
    /// 行列積
    pub fn mul(&self, rhs: &Self) -> Matrix {
        if self.col() != rhs.row() {
            error!("A's row count is {}, but B's column count is {}, so these product can't be defined.", self.col(), rhs.row() );
            exit(1);
        }
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
    /// 掃き出し法によって行列を掃き出す。
    #[allow(dead_code)]
    pub fn sweeped(&self) -> Regularity {
        self.sweeped_verbose(false)
    }

    /// 掃き出し法によって行列を掃き出す。途中結果も出力する。
    pub fn sweeped_verbose(&self, verbose:bool) -> Regularity {
        let mut sweeped = self.clone();
        for r in 0..min(self.row(), self.col()) {
            // アンカーが0になることを防ぐべく、0でない行列を探す。
            {
                let mut r_1 = r;
                while sweeped.mat[r_1][r] == Fraction::zero() {
                    r_1 += 1;
                    // もしそれ以上掃き出しようがなければ、その地点での行列を返して終了する。
                    if r_1 == sweeped.row() { return Regularity::Sigular(sweeped); }
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
            if verbose { println!("->\n{}", sweeped); }
        }
        return Regularity::Regular(sweeped);
    }
    /// 掃き出し法によって逆行列を求める。
    #[allow(dead_code)]
    pub fn inversed(&self) -> Matrix{
        self.inversed_verbose(false)
    }
    /// 掃き出し法によって逆行列を求める。
    /// もし与えられた行列が正則でなければ、エラーを出力して終了する。
    pub fn inversed_verbose(&self, verbose:bool) -> Matrix{
        if self.col() != self.row() {
            error!("The given matrix A is not square, so there is no inversed matrix of A.");
            exit(1);
        }
        match self.push_back_identity().sweeped_verbose(verbose){
            Regularity::Regular(mat) => mat.pop_identity(),
            Regularity::Sigular(_mat) => {
                error!("The given matrix A is singular, so there is no inversed matrix of A.");
                exit(1);
            }
        }

    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.mat {
            for cell in row {
                write!(f, "{:5} ", cell).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}