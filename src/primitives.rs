use std::fmt;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1);
        write!(f, "( {} {} )", self.2, self.3)
    }
}

impl Matrix {
    pub fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

pub fn prim_main() {
    let test = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("{}", test);
    println!("Transpose:\n{}", test.transpose());
}

