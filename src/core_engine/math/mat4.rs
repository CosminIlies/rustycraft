pub struct Matrix4{
    pub mat: [[f32;4]; 4]
}

impl Matrix4{
    pub fn new(mat: [[f32; 4]; 4]) -> Matrix4{
        Matrix4{
            mat
        }
    }

    pub fn identity() -> Matrix4{
        Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    pub fn zero() -> Matrix4{
        Matrix4::new([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ])
    }

    pub fn multiply_by(&mut self, other:&Matrix4){
        let mut mult = Matrix4::zero();

        for i in 0..4{
            for j in 0..4{
                for k in 0..4{

                    mult.mat[i][j] += self.mat[i][k] * other.mat[k][j];

                }
            }
        }

        self.mat = mult.mat;
    }


    pub fn as_array(&mut self) -> [[f32; 4]; 4] {
        self.mat
    }

    pub fn print(&self){
        for x in 0..4{
            for y in 0..4{
                print!("{}", self.mat[x][y]);
            }
            println!();
        }
    }
}