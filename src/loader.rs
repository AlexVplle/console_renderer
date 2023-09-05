pub mod loader {
    use std::{fs, str::{Lines, SplitWhitespace}, path::Path, usize};

    pub struct Vec2 {
        x: f64,
        y: f64,
    }

    #[derive(Clone)]
    pub struct Vec3 {
        x: f64,
        y: f64,
        z: f64,
    }

    pub fn load_obj(
        file_path: String, 
        vertices_array: &mut Vec<Vec3>,
        _uvs_array: Vec<Vec2>,
        _normals_array: Vec<Vec3>,
    ) -> bool {
        let mut temp_vertices_array : Vec<Vec3> = vec![];
        let mut temp_uvs_array : Vec<Vec2> = vec![];
        let mut temp_normals_array : Vec<Vec3> = vec![];
        
        let path : &Path = Path::new(&file_path);

        let is_valid_file : bool = if let Some(extension) = path.extension() {
            extension == "obj"
        } else {
            false
        };

        if !is_valid_file {
            println!("Your file is not an OBJ file");
            return false;
        }

        let content : String = match fs::read_to_string(path) {
            Ok(result) => result,
            Err(_) => {
                println!("Problem to read the file");
                return false;
            }
        };

        for line in content.lines() {
            let mut line_iter : SplitWhitespace = line.split_whitespace();
            if let Some(identifier) = line_iter.next() {
                match identifier {
                    "v" => {
                        if let (Some(x_str), Some(y_str), Some(z_str)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
                            if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>()) {
                                temp_vertices_array.push(Vec3 { x, y, z })
                            }
                        }
                    },

                    "vt" => {
                        if let (Some(x_str), Some(y_str)) = (line_iter.next(), line_iter.next()) {
                            if let (Ok(x), Ok(y)) = (x_str.parse::<f64>(), y_str.parse::<f64>()) {
                                temp_uvs_array.push(Vec2 { x, y })
                            }
                        }
                    },

                    "vn" => {
                        if let (Some(x_str), Some(y_str), Some(z_str)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
                            if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>()) {
                                temp_normals_array.push(Vec3 { x, y, z });
                            }
                        }
                    },

                    "f" => {
                        for point in line_iter {
                            let split = point.split("/");
                            let [vertice, texture, normal] = [&split[0], &split[1], &split[2]];
                            if let Ok(index) = vertice.parse::<usize>() {
                                vertices_array.push(temp_vertices_array[index - 1].clone())
                            }
                        }
                    },
                    _ => {}
                }
            }
        };
        true
    }
}

