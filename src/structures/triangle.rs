use nalgebra::Vector3;

pub struct Triangle {
    pub vertices: [Vector3<f64>; 3],
    pub normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(vertices: [Vector3<f64>; 3]) -> Self {
        let normal : Vector3<f64> = Triangle::calculate_normal(vertices);
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
}
