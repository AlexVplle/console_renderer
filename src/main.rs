mod loader;
mod structures;
mod args;

use clap::Parser;
use crossterm::terminal;
use nalgebra::Matrix4;
use std::f64::consts;

use crate::args::Args;

fn main() {
    let args : Args = Args::parse();
    let mut vertices_array = vec![];
    let mut uvs_array = vec![];
    let mut normals_array = vec![];
    if !loader::load_obj(args.file_path ,&mut vertices_array, &mut uvs_array, &mut normals_array) {
        return;
    }
    const F_NEAR : f64 = 0.1;
    const F_FAR : f64 = 1000.0;
    const F_FOV : f64 = 90.0;
    
    let window_size = match terminal::window_size() {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Problem about getting window size");
            return;
        }
    };

    let f_aspect_ratio : f64 = window_size.height as f64 / window_size.width as f64;
    let f_fov_rad : f64 = 1.0 / (F_FOV * 0.5 / 180.0 * consts::PI).tan();

    let mat = Matrix4::new(f_aspect_ratio * f_fov_rad, 0.0, 0.0, 0.0,
                           0.0, f_fov_rad, 0.0, 0.0,
                           0.0, 0.0, F_FAR / (F_FAR - F_NEAR), 1.0,
                           0.0, 0.0, (-F_FAR * F_NEAR) / (F_FAR - F_NEAR), 0.0);
    dbg!(mat);
}

