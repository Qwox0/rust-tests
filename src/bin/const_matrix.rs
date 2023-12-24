#[derive(Debug)]
pub struct Matrix<const W: usize, const H: usize>(Box<[[f32; W]; H]>);

impl<const W: usize, const H: usize> From<Box<[[f32; W]; H]>> for Matrix<W, H> {
    fn from(value: Box<[[f32; W]; H]>) -> Self {
        Self(value)
    }
}

impl<const W: usize, const H: usize> From<[[f32; W]; H]> for Matrix<W, H> {
    fn from(value: [[f32; W]; H]) -> Self {
        Self(Box::new(value))
    }
}

impl<const W: usize, const H: usize> Default for Matrix<W, H> {
    fn default() -> Self {
        Self(Box::new([[0.0; W]; H]))
    }
}

impl<const W: usize, const H: usize> Matrix<W, H> {
    pub fn dot(&self, vec: &[f32; W]) -> Box<[f32; H]> {
        let mut out = Box::new([0.0; H]);
        for (idx, row) in self.0.iter().enumerate() {
            out[idx] = row.into_iter().zip(vec).map(|(a, b)| a * b).sum()
        }
        out
    }
}

fn main() {
    let mat = Matrix(Box::new([[1.0f32, 2.0], [3.0, 4.0], [5.0, 6.0]]));
    let vec = Box::new([2.0, 0.1]);

    let dot = mat.dot(&vec);

    println!("Matrix: {:#?}", mat);
    println!("Vector: {:?}", vec);
    println!("Output: {:?}", dot);
}
