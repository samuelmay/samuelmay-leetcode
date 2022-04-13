use std::fmt;

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "({}, {})", self.0, self.1)?;
		writeln!(f, "({}, {})", self.2, self.3)
	}
}

fn transpose(matrix: Matrix) -> Matrix {
	let Matrix (a, b, c, d ) = matrix;
	Matrix(a, c, b, d)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}