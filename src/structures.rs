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

pub struct RotationMatrix {
    pub clockwise: Matrix4<f64>,
    pub counterclockwise: Matrix4<f64>,
}

pub struct RotationMatrixAxis {
    pub x_rotation_matrix: RotationMatrix,
    pub y_rotation_matrix: RotationMatrix,
    pub z_rotation_matrix: RotationMatrix,
}

impl RotationMatrixAxis {
    pub fn new(step: f64) -> Self {
        let x_rotation_matrix_counterclockwise: Matrix4<f64> = Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            step.cos(),
            -step.sin(),
            0.0,
            0.0,
            step.sin(),
            step.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let x_rotation_matrix_clockwise: Matrix4<f64> = Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            step.cos(),
            step.sin(),
            0.0, // Inverser les valeurs sin et cos
            0.0,
            -step.sin(),
            step.cos(),
            0.0, // Inverser les valeurs sin et cos
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let x_rotation_matrix: RotationMatrix = RotationMatrix {
            clockwise: x_rotation_matrix_clockwise,
            counterclockwise: x_rotation_matrix_counterclockwise,
        };

        let y_rotation_matrix_counterclockwise: Matrix4<f64> = Matrix4::new(
            step.cos(),
            0.0,
            -step.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            step.sin(),
            0.0,
            step.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let y_rotation_matrix_clockwise: Matrix4<f64> = Matrix4::new(
            step.cos(),
            0.0,
            step.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -step.sin(),
            0.0,
            step.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let y_rotation_matrix: RotationMatrix = RotationMatrix {
            clockwise: y_rotation_matrix_clockwise,
            counterclockwise: y_rotation_matrix_counterclockwise,
        };

        let z_rotation_matrix_counterclockwise: Matrix4<f64> = Matrix4::new(
            step.cos(),
            -step.sin(),
            0.0,
            0.0,
            step.sin(),
            step.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let z_rotation_matrix_clockwise: Matrix4<f64> = Matrix4::new(
            step.cos(),
            step.sin(),
            0.0,
            0.0, // Inverser les valeurs sin et cos
            -step.sin(),
            step.cos(),
            0.0,
            0.0, // Inverser les valeurs sin et cos
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let z_rotation_matrix: RotationMatrix = RotationMatrix { clockwise: z_rotation_matrix_clockwise, counterclockwise: z_rotation_matrix_counterclockwise };

        RotationMatrixAxis {
            x_rotation_matrix,
            y_rotation_matrix,
            z_rotation_matrix,
        }
    }
}
