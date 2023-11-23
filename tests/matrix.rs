extern crate bird_mat_calc;
use bird_mat_calc::matrix::Matrix;

#[test]
fn test_pushed_identity() {
    let identity = Matrix::from_vector_of_str(
        vec![
            vec!["1", "0"],
            vec!["0", "1"],
        ]
    );
    let matrix = Matrix::from_vector_of_str(
        vec![
            vec!["1", "2"],
            vec!["4", "7"],
        ]
    );
    let matrix_pushed_identity = Matrix::from_vector_of_str(
        vec![
            vec!["1", "2", "1", "0"],
            vec!["4", "7", "0", "1"],
        ]
    );
    assert_eq!(matrix.push_back_identity(), matrix_pushed_identity);
    assert_eq!(matrix_pushed_identity.pop_identity(), identity);
}

#[test]
fn test_inversed() {
    let identity = Matrix::from_vector_of_str(
        vec![
            vec!["1", "0"],
            vec!["0", "1"],
        ]
    );
    let matrix = Matrix::from_vector_of_str(
        vec![
            vec!["1", "2"],
            vec!["4", "7"],
        ]
    );
    assert_eq!(matrix.inversed().mul(&matrix), identity);
    assert_eq!(matrix.mul(&matrix.inversed()), identity);
}


#[test]
fn test_matrix_mul() {
    let m1 = Matrix::from_vector_of_str(
        vec![
            vec!["1", "3", "5", "1", "5"],
            vec!["6", "5", "3", "0", "1"],
            vec!["2", "1", "3", "0", "3"],
        ]
    );
    let m2 = Matrix::from_vector_of_str(
        vec![
            vec!["1", "2"],
            vec!["1", "3"],
            vec!["4", "5"],
            vec!["1", "-1"],
            vec!["-3/5", "3"],
        ]
    );
    let m3 = Matrix::from_vector_of_str(
        vec![
            vec!["22", "50"],
            vec!["224/10", "45"],
            vec!["132/10", "31"]
        ]
    );
    assert_eq!(m1.mul(&m2), m3);
    
}

#[test]
fn test_inversed_complex() {
    let identity = Matrix::from_vector_of_str(
        vec![
            vec!["1", "0", "0", "0"],
            vec!["0", "1", "0", "0"],
            vec!["0", "0", "1", "0"],
            vec!["0", "0", "0", "1"],
        ]
    );
    let matrix = Matrix::from_vector_of_str(
        vec![
            vec!["1", "2", "4", "9"],
            vec!["3", "7", "8", "1"],
            vec!["2", "1", "6", "9"],
            vec!["6", "7", "1", "-4"],
        ]
    );
    assert_eq!(matrix.inversed().mul(&matrix), identity);
    assert_eq!(matrix.mul(&matrix.inversed()), identity);
}

