use std::{fs, str::SplitWhitespace, path::Path, usize};
use crate::structures::{Vec2, Vec3};


pub fn load_obj(
    file_path: String, 
    vertices_array: &mut Vec<Vec3>,
    uvs_array: &mut Vec<Vec2>,
    normals_array: &mut Vec<Vec3>,
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
        eprintln!("Your file is not an OBJ file");
        return false;
    }

    let content : String = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Problem to read the file");
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
                        let mut split = point.split("/");
                        if let (Some(vertice_str), Some(texture_str), Some(normal_str)) = (split.next(), split.next(), split.next()) {
                            if let (Ok(vertice), Ok(texture), Ok(normal)) = (vertice_str.parse::<usize>(), texture_str.parse::<usize>(), normal_str.parse::<usize>()) {
                                vertices_array.push(temp_vertices_array[vertice - 1].clone());
                                uvs_array.push(temp_uvs_array[texture - 1].clone());
                                normals_array.push(temp_normals_array[normal - 1].clone());
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    };
    true
}
