use std::str::FromStr;
use fraction::Fraction;

pub type MatrixArray = Vec::<Vec::<Fraction>>;
#[derive(Clone, PartialEq, Debug)]
pub struct Matrix{
    pub mat: MatrixArray,
    row: usize,
    col: usize,
}

impl Matrix {
    /// 標準入力から行列を読み取る。もし1行空行が挟まれば、その地点で読み取りを終了する。
    /// 
    /// #TODO: これを一般的なストリームに拡張したい...。BufReadだとstd::io::stdin()と互換性なし。どうすればいいんだ！？
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
    /// 文字列の二次元配列から、Fractionからなる行列を生成する。
    pub fn from_vector_of_str(array:Vec<Vec<&str>>) -> Matrix {
        Matrix::as_matrix(array.iter().map(
            |v| v.iter().map(
                |x| Fraction::from_str(x).expect("failed to parse as fraction")
            ).collect()
        ).collect())
    }
    /// 行列に対して、行方向に新たに正方単位行列を追加したものを返す。
    pub fn push_back_identity(&self) -> Matrix {
        let mut matrix = Matrix::zero(self.row(), self.col() + self.row());
        for r in 0..self.row() {
            for c in 0..self.col() {
                matrix.mat[r][c] = self.mat[r][c];
            }
        }
        for r in 0..self.col() {
            matrix.mat[r][r + self.col()] = Fraction::from(1);
        }
        return matrix;
    }
    /// 行方向に見て、後方にある正方行列を取り出し、新たな正方行列を得る。
    pub fn pop_identity(&self) -> Matrix {
        assert!(self.row() < self.col());
        let mut matrix = Matrix::zero(self.row(), self.row());
        for r in 0..matrix.row() {
            for c in 0..matrix.col() {
                matrix.mat[r][c] = self.mat[r][matrix.row() + c];
            }
        }
        assert_eq!(matrix.row(), matrix.col());
        return matrix;
    }

    /// `row`行`col`列の零行列を返す
    pub fn zero(row:usize, col:usize) -> Self{
        Matrix {
            mat: vec![vec![Fraction::from(0); col]; row],
            row, col
        }
    }
    /// Fractionの二次元配列で表される行列を`Matrix`で表したものを返す。
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

}