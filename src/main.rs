mod args;
mod loader;
mod structures;

use std::{f64::consts, time::Duration};

use clap::Parser;
use console_engine::{pixel, ConsoleEngine};
use crossterm::{
    event::{self, KeyCode},
    terminal::{self, WindowSize},
};
use nalgebra::{Matrix4, Vector3};
use structures::{Camera, RotationMatrixAxis, Triangle};

use crate::args::Args;

fn main() {
    let args: Args = Args::parse();

    let vertices_array = match loader::load_obj(args.file_path) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut triangle_array: Vec<Triangle> = vec![];
    for chunk in vertices_array.chunks(3) {
        triangle_array.push(Triangle::new([chunk[0], chunk[1], chunk[2]]));
    }

    let mut app: App = match App::new(triangle_array) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    if let Err(err) = app.main_loop() {
        eprintln!("{}", err);
        return;
    };
}

struct App {
    engine: ConsoleEngine,
    window_size: WindowSize,
    triangle_array: Vec<Triangle>,
    camera: Camera,
    scale_factor: f64,
    rotation_matrix_axis: RotationMatrixAxis,
}

impl App {
    fn new(mut triangle_array: Vec<Triangle>) -> Result<Self, String> {
        // if let Err(_) = terminal::enable_raw_mode() {
        //     return Err(String::from("Could not turn on Raw mode"));
        // };
        let window_size: WindowSize = match terminal::window_size() {
            Ok(result) => result,
            Err(_) => return Err(String::from("Problem about getting window size")),
        };
        let engine: ConsoleEngine = match console_engine::ConsoleEngine::init(
            window_size.columns as u32,
            window_size.rows as u32,
            30,
        ) {
            Ok(result) => result,
            Err(_) => return Err(String::from("Problem about creating console engine")),
        };
        let aspect_ratio: f64 = window_size.height as f64 / window_size.width as f64;

        let camera_position: Vector3<f64> = Vector3::zeros();
        let camera = Camera::new(camera_position, 0.1, 1000.0, 90.0, aspect_ratio);

        let scale_factor: f64 = 1.0;
        let rotation_matrix_axis: RotationMatrixAxis = RotationMatrixAxis::new(consts::PI / 30.0);

        let translation_vector: Vector3<f64> = Vector3::new(0.0, 0.0, 3.0);
        for triangle in &mut triangle_array {
            triangle.translate(translation_vector);
        }

        Ok(App {
            engine,
            triangle_array,
            window_size,
            camera,
            scale_factor,
            rotation_matrix_axis,
        })
    }

    fn main_loop(&mut self) -> Result<(), String> {
        loop {
            if event::poll(Duration::from_millis(500))
                .expect("Error occurred during event listener")
            {
                match event::read().expect("Error during reading events") {
                    event::Event::Key(key) => match key.code {
                        KeyCode::Char('a') => self.update_rotation(
                            self.rotation_matrix_axis.z_rotation_matrix.counterclockwise,
                        ),
                        KeyCode::Char('e') => self
                            .update_rotation(self.rotation_matrix_axis.z_rotation_matrix.clockwise),
                        KeyCode::Char('q') => self
                            .update_rotation(self.rotation_matrix_axis.y_rotation_matrix.clockwise),
                        KeyCode::Char('d') => self.update_rotation(
                            self.rotation_matrix_axis.y_rotation_matrix.counterclockwise,
                        ),
                        KeyCode::Char('r') => self
                            .update_rotation(self.rotation_matrix_axis.x_rotation_matrix.clockwise),
                        KeyCode::Char('f') => self.update_rotation(
                            self.rotation_matrix_axis.x_rotation_matrix.counterclockwise,
                        ),
                        KeyCode::Char('z') => self.scale_factor += 0.1,
                        KeyCode::Char('s') => self.scale_factor -= 0.1,
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.draw()?;
        }
    }

    fn draw(&mut self) -> Result<(), String> {
        self.engine.wait_frame();
        self.engine.clear_screen();
        let center_x = self.window_size.columns as f64 / 2.0;
        let center_y = self.window_size.rows as f64 / 2.0;
        for triangle in &mut self.triangle_array {
            if triangle
                .normal
                .dot(&(triangle.vertices[0] - self.camera.position))
                < 0.0
            // if true
            {
                let mut projected_triangle: Triangle = triangle.clone();
                projected_triangle.multiply_matrix_vector(self.camera.projection_matrix);

                for i in 0..3 {
                    projected_triangle.vertices[i].x =
                        (projected_triangle.vertices[i].x * self.scale_factor * center_x)
                            + center_x;
                    projected_triangle.vertices[i].y =
                        (projected_triangle.vertices[i].y * self.scale_factor * center_y)
                            + center_y;
                }

                self.engine.triangle(
                    projected_triangle.vertices[0].x as i32,
                    projected_triangle.vertices[0].y as i32,
                    projected_triangle.vertices[1].x as i32,
                    projected_triangle.vertices[1].y as i32,
                    projected_triangle.vertices[2].x as i32,
                    projected_triangle.vertices[2].y as i32,
                    pixel::pxl('#'),
                );
                self.engine.draw();
            }
        }
        Ok(())
    }

    fn update_rotation(&mut self, rotation_matrix: Matrix4<f64>) {
        let center = self.find_center();
        for triangle in &mut self.triangle_array {
            triangle.translate(-center);
            triangle.multiply_matrix_vector(rotation_matrix);
            triangle.translate(center);
            triangle.update_normal();
        }
    }

    fn find_center(&self) -> Vector3<f64> {
        let mut center = Vector3::zeros();
        let num_points = self.triangle_array.len() * 3;

        for triangle in &self.triangle_array {
            for point in triangle.vertices {
                center += point;
            }
        }

        center / num_points as f64
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Err(err) = terminal::disable_raw_mode() {
            eprintln!("Could not turn off Raw mode: {}", err);
        };
    }
}
