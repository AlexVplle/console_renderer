use nalgebra::Matrix4;
use std::f64::consts;

pub struct Camera {
    pub near: f64,
    pub far: f64,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub projection_matrix: Matrix4<f64>,
}

impl Camera {
    pub fn new(near: f64, far: f64, fov_angle: f64, aspect_ratio: f64) -> Self {
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
            near,
            far,
            fov,
            aspect_ratio,
            projection_matrix,
        }
    }
}
