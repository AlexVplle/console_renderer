use nalgebra::Vector3;

pub struct Light {
    pub position: Vector3<f64>,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Vector3<f64>, intensity: f64) -> Self {
        Self { position, intensity }
    }
}