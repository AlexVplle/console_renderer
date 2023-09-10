mod loader;
mod structures;
mod args;

use clap::Parser;
use console_engine::pixel;
use crossterm::terminal;
use nalgebra::{Matrix4, Vector2, Vector3};
use std::f64::consts;

use crate::args::Args;

fn main() {
    let args : Args = Args::parse();
    let window_size = match terminal::window_size() {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Problem about getting window size");
            return;
        }
    };

    let mut engine = match console_engine::ConsoleEngine::init(window_size.columns as u32, window_size.rows as u32, 60) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Problem about creating console engine");
            return;
        }
    };

    let mut vertices_array : Vec<Vector3<f64>> = vec![];
    let mut uvs_array : Vec<Vector2<f64>> = vec![];
    let mut normals_array : Vec<Vector3<f64>> = vec![];
    if !loader::load_obj(args.file_path ,&mut vertices_array, &mut uvs_array, &mut normals_array) {
        return;
    }

    let mut triangle_array : Vec<[Vector3<f64>; 3]> = vec![];
    for chunk in vertices_array.chunks(3) {
        triangle_array.push([chunk[0], chunk[1], chunk[2]]);
    }

    const F_NEAR : f64 = 0.1;
    const F_FAR : f64 = 1000.0;
    const F_FOV : f64 = 90.0;

    let f_aspect_ratio : f64 = window_size.height as f64 / window_size.width as f64;
    let f_fov_rad : f64 = 1.0 / (F_FOV * 0.5 / 180.0 * consts::PI).tan();

    let projection_matrix : Matrix4<f64> = Matrix4::new(
        f_aspect_ratio * f_fov_rad, 0.0, 0.0, 0.0,
        0.0, f_fov_rad, 0.0, 0.0,
        0.0, 0.0, F_FAR / (F_FAR - F_NEAR), 1.0,
        0.0, 0.0, (-F_FAR * F_NEAR) / (F_FAR - F_NEAR), 0.0
    );
    let mut i : f64 = 0.0;

    loop {
        i += 0.00001;
        engine.wait_frame();
        engine.clear_screen();

        // let rotation_matrix_z : Matrix4<f64> = Matrix4::new(
        //     i.cos(), -i.sin(), 0.0, 0.0, 
        //     i.sin(), i.cos(), 0.0, 0.0, 
        //     0.0, 0.0, 1.0, 0.0, 
        //     0.0, 0.0, 0.0, 1.0
        // );

        let rotation_matrix_x : Matrix4<f64> = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 
            0.0, i.cos(), -i.sin(), 0.0, 
            -i.sin(), 0.0, i.cos(), 0.0, 
            0.0, 0.0, 0.0, 1.0
        );
        // dbg!(rotation_matrix_x);

        for triangle in &mut triangle_array {
            triangle[0] = multiply_matrix_vector(triangle[0], rotation_matrix_x);
            triangle[1] = multiply_matrix_vector(triangle[1], rotation_matrix_x);
            triangle[2] = multiply_matrix_vector(triangle[2], rotation_matrix_x);

            // triangle[0] = multiply_matrix_vector(triangle[0], rotation_matrix_z);
            // triangle[1] = multiply_matrix_vector(triangle[1], rotation_matrix_z);
            // triangle[2] = multiply_matrix_vector(triangle[2], rotation_matrix_z);

            let first_point = multiply_matrix_vector(triangle[0], projection_matrix);
            let second_point = multiply_matrix_vector(triangle[1], projection_matrix);
            let third_point = multiply_matrix_vector(triangle[2], projection_matrix);
            engine.fill_triangle((first_point.x * (window_size.columns as f64)) as i32 / 2, (first_point.y * (window_size.rows as f64)) as i32 / 2, (second_point.x * (window_size.columns as f64)) as i32 / 2, (second_point.y * (window_size.rows as f64)) as i32 / 2, (third_point.x * (window_size.columns as f64)) as i32 / 2, (third_point.y * (window_size.rows as f64)) as i32 / 2, pixel::pxl('#'));
        }
        if engine.is_key_pressed(console_engine::KeyCode::Esc) {
            break;
        }
        engine.draw();
    }
}

fn multiply_matrix_vector(vector : Vector3<f64>, matrix : Matrix4<f64>) -> Vector3<f64> { 
    let mut new_vector : Vector3<f64> = Vector3::new(
        vector.x * matrix[(0, 0)] + vector.y * matrix[(1, 0)] + vector.z * matrix[(2, 0)] + matrix[(3, 0)],
        vector.x * matrix[(0, 1)] + vector.y * matrix[(1, 1)] + vector.z * matrix[(2, 1)] + matrix[(3, 1)],
        vector.x * matrix[(0, 2)] + vector.y * matrix[(1, 2)] + vector.z * matrix[(2, 2)] + matrix[(3, 2)],
    );
    let w : f64 = vector.x * matrix[(0, 3)] + vector.y * matrix[(1, 3)] + vector.z * matrix[(2, 3)] + matrix[(3, 3)];
    if w != 0.0 {
        new_vector.x /= w;
        new_vector.y /= w;
        new_vector.z /= w;
    }
    return new_vector;
}

fn draw_line(x_a : i64, y_a : i64, x_b : i64, y_b : i64, character : char) -> () {
    let delta_x : i64 = x_b - x_a;
    let delta_y : i64 = y_b - y_a;
    let m : f64 = delta_y as f64 / delta_x as f64;
    for i in 0..delta_x {
        let x : i64 = x_a + i;
        let y : i64 = y_a + (m * i as f64).ceil() as i64;
    }
}

fn draw_pixel(x : i64, y : i64) {

}
