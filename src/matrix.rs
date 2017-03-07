use std::fmt;
use std::ops::{Add, Sub, Mul};
use std::iter::Iterator;

/// 4xN matrices
#[derive(Clone)]
pub struct Matrix {
    /// Vec of the columns of the matrix
    cols: Vec<[f64; 4]>
}

/// All operations using indexes are 0-based.
impl Matrix {
    /// Make a 4xN matrix.
    pub fn new(columns: Vec<[f64; 4]>) -> Matrix {
        Matrix { cols: columns }
    }

    pub fn with_capacity(cols: usize, val: f64) -> Matrix {
        Matrix::new(vec![[val; 4]; cols])
    }

    /// Make an empty (4x0) matrix.
    pub fn empty() -> Matrix {
        Matrix::new(vec![])
    }

    /// Make the column matrix representing the origin.
    pub fn origin() -> Matrix {
        Matrix::new(vec![[0.0, 0.0, 0.0, 1.0]])
    }

    /// Make a 4x4 matrix given each cell value (parameters listed row-by-row).
    pub fn new4x4(
        a: f64, b: f64, c: f64, d: f64,
        e: f64, f: f64, g: f64, h: f64,
        i: f64, j: f64, k: f64, l: f64,
        m: f64, n: f64, o: f64, p: f64) -> Matrix {
        Matrix {
            cols: vec![
                [a, e, i, m],
                [b, f, j, n],
                [c, g, k, o],
                [d, h, l, p]
            ]
        }
    }

    /// Make a 4x4 identity matrix
    pub fn identity() -> Matrix {
        Matrix::new(vec![
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]])
    }

    /// Make a translation matrix for translation by (dx, dy, dz)
    pub fn translation(dx: f64, dy: f64, dz: f64) -> Matrix {
        Matrix::new4x4(
            1.0, 0.0, 0.0, dx,
            0.0, 1.0, 0.0, dy,
            0.0, 0.0, 1.0, dz,
            0.0, 0.0, 0.0, 1.0)
    }

    /// Make a 4x4 dilation matrix dilating by `s` in
    /// x, y, and z.
    pub fn dilation(s: f64) -> Matrix {
        s * &Matrix::identity()
    }

    /// Make a 4x4 dilation matrix dilating by `sx` in
    /// x, `sy`, in y, and `sz` in z.
    pub fn dilation_xyz(sx: f64, sy: f64, sz: f64) -> Matrix {
        Matrix::new4x4(
            sx, 0.0, 0.0, 0.0,
            0.0, sy, 0.0, 0.0,
            0.0, 0.0, sz, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn rotation_about_x(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);
        Matrix::new4x4(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, -sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn rotation_about_y(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);
        Matrix::new4x4(
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    /// Make a 4x4 rotation matrix for a rotation of `angle` radians
    /// about the z axis.
    pub fn rotation_about_z(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);
        Matrix::new4x4(
            cos, -sin, 0.0, 0.0,
            sin, cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    /// Make a 4x4 shear matrix for a shear in the XY plane.
    pub fn shear_2d(dx: f64, dy: f64) -> Matrix {
        Matrix::new4x4(
            1.0, dx, 0.0, 0.0,
            dy, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    /// Get an array of the elements in column `colnum`.
    pub fn col(&self, colnum: usize) -> [f64; 4] {
        let width = self.cols.len();
        if colnum > width {
            panic!("Attempted to get column {} of a matrix of width {}", colnum, width);
        }
        self.cols[colnum]
    }

    /// Get a Vec of the elements in column `colnum`.
    pub fn col_vec(&self, colnum: usize) -> Vec<f64> {
        let width = self.cols.len();
        if colnum > width {
            panic!("Attempted to get column {} of a matrix of width {}", colnum, width);
        }
        let col = &self.cols[colnum];
        vec![col[0], col[1], col[2], col[3]] // TODO: Into<Vec<T>>?
    }

    /// Push a column to the right side of `self`.
    pub fn push_col(&mut self, col: [f64; 4]) {
        self.cols.push(col)
    }

    /// Push each column of `m` to `self`
    pub fn append(&mut self, m: Matrix) {
        for col in 0..m.width() {
            self.push_col(m.col(col));
        }
    }

    /// Push an edge, i.e. two points, to `self` (think of `self` as an edge list).
    pub fn push_edge(&mut self, pt0: [f64; 4], pt1: [f64; 4]) {
        self.push_col(pt0);
        self.push_col(pt1);
    }

    /// Get a vector of entries in row `rownum`.
    pub fn row(&self, rownum: usize) -> Vec<f64> {
        if rownum > 3 {
            panic!("Attempted to get row {} of a matrix of height 4", rownum);
        }
        let mut items = vec![];
        for column in &self.cols {
            items.push(column[rownum]);
        }
        items
    }

    /// Get the entry at row `row` and column `col`.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.cols[col][row]
    }


    /// Set the entry at row `row` and column `col` to `val`.
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.cols[col][row] = val;
    }

    /// Get the width of the matrix.
    pub fn width(&self) -> usize {
        self.cols.len()
    }
}

// ref plus ref
impl<'a, 'b> Add<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: &Matrix) -> Matrix {
        let mut cols = self.cols.clone();
        for (vcol, rcol) in cols.iter_mut().zip(rhs.cols.iter()) {
            vcol[0] += rcol[0];
            vcol[1] += rcol[1];
            vcol[2] += rcol[2];
            vcol[3] += rcol[3];
        }
        Matrix::new(cols)
    }
}

// owned plus ref
impl<'a> Add<&'a Matrix> for Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: &Matrix) -> Matrix {
        &self + rhs
    }
}

// ref plus owned
impl<'a> Add<Matrix> for &'a Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: Matrix) -> Matrix {
        self + &rhs
    }
}

// owned plus owned
impl Add<Matrix> for Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: Matrix) -> Matrix {
        &self + &rhs
    }
}

// TODO: add owned version of impls for Sub and Mul (as done with Add above)
impl<'a, 'b> Sub<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn sub(self, rhs: &Matrix) -> Matrix {
        let mrhs = rhs * -1.0;
        self + &mrhs
    }
}

impl<'a, 'b> Mul<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        let mut m = Matrix::with_capacity(rhs.width(), 0.0);
        for i in 0..4 {
            for j in 0..rhs.width() {
                let val: f64 = dot_product_refs(self.row(i).iter(), rhs.col(j).iter());
                m.set(i, j, val);
            }
        }
        m
    }
}

fn dot_product_refs<'a, 'b, T: Iterator<Item=&'a f64>, U: Iterator<Item=&'b f64>>(v: T, u: U) -> f64 {
    let mut sum = 0.0;
    for (&a, &b) in v.zip(u) {
        sum += a * b;
    }
    sum
}

fn dot_product<T: Iterator<Item=f64>, U: Iterator<Item=f64>>(v: T, u: U) -> f64 {
    let mut sum = 0.0;
    for (a, b) in v.zip(u) {
        sum += a * b;
    }
    sum
}

fn scale_matrix(scalar: f64, mat: &Matrix) -> Matrix {
    let mut result = Matrix::with_capacity(mat.width(), 0.0);
    for row in 0..4 {
        for col in 0..mat.width() {
            result.set(row, col, scalar * mat.get(row, col));
        }
    }
    result
}

impl<'a> Mul<f64> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix {
        scale_matrix(rhs, self)
    }
}

impl<'a> Mul<&'a Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        scale_matrix(self, rhs)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        for row in 0..4 {
            s.push_str(match row {
                0 => "/ ",
                3 => "\\ ",
                _ => "| "
            });
            for col in 0..self.width() {
                s.push_str(&format!("{} ", self.get(row, col)));
            }
            s.push_str(match row {
                0 => "\\\n",
                3 => "/\n",
                _ => "|\n"
            });
        }
        write!(f, "{}", s)
    }
}
