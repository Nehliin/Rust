
use std;

#[derive(Debug)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {

    pub fn new(number_of_columns: usize, number_of_rows: usize) -> Matrix {
        let mut rows:Vec<Vec<f64>> = Vec::with_capacity(number_of_rows);
        for _ in 0..number_of_rows {
            rows.push(vec![0.0;number_of_columns]);
        }
        Matrix(rows)
    }
    pub fn get_rows(&self) -> &Vec<Vec<f64>> {
        return &self.0;
    }

    //vec as matrix?
    //impl transpose?
    pub fn transpose(&self) -> Matrix {
        let mut new_rows: Vec<Vec<f64>> = Vec::with_capacity(self.0[0].len());
        let mut new_row:Vec<f64> = Vec::with_capacity(self.0.len());
        for i in 0..self.0[0].len() { // col length
            for row in &self.0 {
                new_row.push(row[i])
            }
            new_rows.push(new_row.clone());
            new_row.clear();
        }
        Matrix(new_rows)
    }
}

impl std::fmt::Display for Matrix {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let transpose = self.transpose();
        let mut print = String::new();
        for row in transpose.get_rows().iter() {
            for element in row {
                print.push_str(element.to_string().as_str());
                print.push_str(", ");
            }
            print.push('\n');
        }
        write!(f, "{}", print)
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        Matrix(self.get_rows().iter().zip(other.get_rows())
            .map(|(v1, v2)| super::vec_adder(&v1, &v2))
            .collect())
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        Matrix(self.get_rows().iter().zip(other.get_rows())
            .map(|(v1, v2)| v1.iter().zip(v2)
                .map(|(u, w)| *u - *w).collect())
            .collect())
    }
}





pub fn matrix_multiply<'a>(matrix: &Matrix, vec: &Vec<f64>) -> Result<Vec<f64>, &'a str> {
    if matrix.get_rows()[0].len() != vec.len() {
        println!("falied multiply: matrix len {}, vec len {}", matrix.get_rows()[0].len(), vec.len());
        return Err("Missmatch of row lenght and vec to be multiplied");
    } else {
        let mut result = vec![0.0;matrix.get_rows().len()];//Vec::with_capacity(matrix.getColumns().len());
        for (index,row) in matrix.get_rows().iter().enumerate() {
            for (m, v) in row.iter().zip(vec) {
                result[index] += *m * *v;
            }
        }

        return Ok(result);
    }
}

pub fn generate_matrix(number_of_columns: &usize, number_of_rows: &usize) -> Matrix {
    let mut matrix = Vec::with_capacity(*number_of_columns);
    for _ in 0..*number_of_rows {
        matrix.push(vec![0.0; *number_of_columns].iter().map(|_| super::normally_distributed()).collect()); // generate column
    }

    return Matrix(matrix);
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn matrix_multiplication() {
        let matrix: Matrix = Matrix(vec![
            vec![1.0, 1.5, 1.2],
            vec![2.0, -3.0, 4.5],
            vec![1.1, 1.2, -1.4]
        ]);
        let wrong_size: Vec<f64> = vec![1.2, 2.3, 3.4, 4.0];
        assert_eq!(true, matrix_multiply(&matrix, &wrong_size).is_err());

        let right_size: Vec<f64> = vec![1.2, 2.3, 3.4];
        let answer: Vec<f64> = vec![8.73, 10.800001, -0.6800003];

        let calculated = matrix_multiply(&matrix, &right_size);

        let calculated = match calculated {
            Ok(result) => result,
            _ => vec![],
        };
        assert_eq!(answer, calculated);
    }

    #[test]
    fn add(){
        let m1 = Matrix(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let m2 = Matrix(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        println!("{}",m1);

        let expected = Matrix(vec![
            vec![2.0, 4.0, 6.0],
            vec![8.0, 10.0, 12.0],
            vec![14.0, 16.0, 18.0]
        ]);
        assert_eq!(expected.get_rows(), (m1 + m2).get_rows());
    }

    #[test]
    fn matrix_generation() {
        let matrix = generate_matrix(&2,&3);
        assert_eq!(2, matrix.get_rows().len());

        let mut prev_col =  vec![2.0;3]; // can never be greater than 1
        for col in matrix.get_rows() {
            assert_eq!(3, col.len());
            assert_ne!(prev_col, *col); // check that columns are unique
            let mut prev = 2.0 as f64; // a value that can never be taken
            for x in col {
                assert_ne!(prev, *x);
                println!("{}",*x);
                assert!( -5.0 <= *x && *x <= 5.0);
                prev = *x;
            }
            prev_col = col.clone();
        }
    }

    #[test]
    fn transpose() {
        let rows = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let input = Matrix(rows);

        let expected = Matrix(vec![
            vec![1.0, 4.0],
            vec![2.0, 5.0],
            vec![3.0, 6.0],
        ]);

        let output = input.transpose();
        assert_eq!(expected.get_rows(), output.get_rows());
    }
}