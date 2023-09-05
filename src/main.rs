mod loader;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vertices_array = vec![];
    let uvs_array = vec![];
    let normals_array = vec![];
    loader::loader::load_obj(args[1].clone() ,&mut vertices_array, uvs_array, normals_array);
}

