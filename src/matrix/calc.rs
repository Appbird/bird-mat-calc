use super::basis::Matrix;
use fraction::{Fraction, Zero};


impl Matrix {
    /// 行列積
    pub fn mul(&self, rhs: &Self) -> Matrix {
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
    /// 掃き出し法によって行列を掃き出す。
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
    /// 掃き出し法によって逆行列を求める。
    pub fn inversed(&self) -> Matrix{
        return self.push_back_identity()
            .sweeped()
            .pop_identity();
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