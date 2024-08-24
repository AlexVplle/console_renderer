use nalgebra::{Vector3, Matrix4};
use std::f64::consts;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Camera {
    pub position: Vector3<f64>,
    pub projection_matrix: Matrix4<f64>,
}

impl Camera {
    pub fn new(
        position: Vector3<f64>,
        near: f64,
        far: f64,
        fov_angle: f64,
        aspect_ratio: f64,
    ) -> Self {
        let fov: f64 = 1.0 / (fov_angle * 0.5 / 180.0 * consts::PI).tan();
        let projection_matrix: Matrix4<f64> = Matrix4::new(
            aspect_ratio * fov,
            0.0,
            0.0,
            0.0,
            0.0,
            fov,
            0.0,
            0.0,
            0.0,
            0.0,
            far / (far - near),
            1.0,
            0.0,
            0.0,
            (-far * near) / (far - near),
            0.0,
        );
        Camera {
            position,
            projection_matrix,
        }
    }
}
