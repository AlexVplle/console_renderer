use nalgebra::{Matrix4, Vector3};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Triangle {
    pub vertices: [Vector3<f64>; 3],
    pub normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(vertices: [Vector3<f64>; 3]) -> Self {
        let normal: Vector3<f64> = Triangle::calculate_normal(vertices);
        Triangle { vertices, normal }
    }

    pub fn translate(&mut self, translation_vector: Vector3<f64>) {
        for vertice in self.vertices.iter_mut() {
            *vertice += translation_vector;
        }
    }

    pub fn update_normal(&mut self) {
        self.normal = Triangle::calculate_normal(self.vertices);
    }

    fn calculate_normal(vertices: [Vector3<f64>; 3]) -> Vector3<f64> {
        let line1: Vector3<f64> = vertices[1] - vertices[0];
        let line2: Vector3<f64> = vertices[2] - vertices[0];
        line1.cross(&line2).normalize()
    }

    pub fn multiply_matrix_vector(&mut self, matrix: Matrix4<f64>) {
        for vertice in &mut self.vertices {
            let mut new_vertice: Vector3<f64> = Vector3::new(
                vertice.x * matrix[(0, 0)]
                    + vertice.y * matrix[(1, 0)]
                    + vertice.z * matrix[(2, 0)]
                    + matrix[(3, 0)],
                vertice.x * matrix[(0, 1)]
                    + vertice.y * matrix[(1, 1)]
                    + vertice.z * matrix[(2, 1)]
                    + matrix[(3, 1)],
                vertice.x * matrix[(0, 2)]
                    + vertice.y * matrix[(1, 2)]
                    + vertice.z * matrix[(2, 2)]
                    + matrix[(3, 2)],
            );
            let w: f64 = vertice.x * matrix[(0, 3)]
                + vertice.y * matrix[(1, 3)]
                + vertice.z * matrix[(2, 3)]
                + matrix[(3, 3)];
            if w != 0.0 {
                new_vertice.x /= w;
                new_vertice.y /= w;
                new_vertice.z /= w;
            }
            *vertice = new_vertice;
        }
    }
}
