
//use rand::prelude::*;
#[derive(Clone)]

pub struct Matrix {
    pub rows:usize,
    pub cols:usize,
    pub data:Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeroes(rows:usize,cols:usize)-> Matrix {
        Matrix {
            rows,
            cols,
            data:vec![vec![0.0; cols];rows]
        }
    }
    pub fn random(rows:usize,cols:usize)-> Matrix {
        let mut res= Matrix::zeroes(rows, cols);
        for i in 0..rows{
            for j in 0..cols{
                res.data[i][j]=(rand::random::<f64>()*2.0)-1.0;
            }
        }
        res
    }
    pub fn from(data:Vec<Vec<f64>>)-> Matrix {
        Matrix {
            rows:data.len(),
            cols:data[0].len(),
            data,
        }
    }
    pub fn multiply(&mut self, other:&Matrix) -> Matrix {
        if self.cols!=other.rows{
            panic!("Attempted to multiply matrix of invalid dimensions");
        }
        let mut res= Matrix::zeroes(self.rows, other.cols);
        for i in 0..self.rows{
            for j in 0..other.cols {
                let mut sum=0.0;
                for k in 0..self.cols {
                    sum=self.data[i][k]*other.data[k][j];
                }
                res.data[i][j]=sum;
            }
        }
        res
    }
    pub fn add(&mut self, other:&Matrix) -> Matrix {
        if self.rows!=other.rows || self.cols!=other.cols{
            panic!("Attempting to add ma  trices of two diff dimensions");
        }
        let mut res= Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }
    pub fn dot_multiply(&mut self, other: &Matrix) -> Matrix {
        if self.rows!=other.rows || self.cols!=other.cols{
            panic!("Attempting to add matrices of two diff dimensions");
        }
        let mut res= Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        res
    }
    pub fn subtract(&mut self, other: &Matrix) ->Matrix {
        if self.rows!=other.rows || self.cols!=other.cols{
            panic!("Attempting to subtract matrices of two diff dimensions");
        }
        let mut res= Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }
    pub fn map(&mut self,function:&dyn Fn(f64)->f64)-> Matrix {
        Matrix::from((self.data)
                         .clone()
                         .into_iter()
                         .map(|row|row.into_iter().map(|value|function(value)).collect())
                         .collect(),
        )

    }
    pub fn transpose(&mut self)->Matrix{
        let mut res=Matrix::zeroes(self.cols,self.rows);
        for i in 0..self.cols{
            for j in 0..self.rows{
                res.data[j][i]=self.data[i][j];
            }
        }
        res
    }

}
