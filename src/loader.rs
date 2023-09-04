use std::{fs, str::{Lines, SplitWhitespace}, path::Path};

struct Vec2 {
    x: f64,
    y: f64,
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Vertex {
    position: Vec3,
    normal: Vec3,
    uvs: Vec2,
}

fn loadOBJ(
    file_path: String, 
    vertices_array: Vec<Vec3>,
    uvs_array: Vec<Vec2>,
    normals_array: Vec<Vec3>,
) -> bool {
    let temp_vertices_array : Vec<Vec3> = vec![];
    let temp_uvs_array : Vec<Vec2> = vec![];
    let temp_normals_array : Vec<Vec3> = vec![];
    
    let path : &Path = Path::new(&file_path);
    if path.extension().unwrap() != "obj" {
        return false;
    }
    let content : String = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(E) => panic!("Problem to read file")
    };
    let lines_iter : Lines = content.lines();
    for line in lines_iter {
        let line_iter : SplitWhitespace = line.split_whitespace();
        if let Some(identifier) = line_iter.next() {
            if identifier == "v" {
                if let (Some(x_str), Some(y_str), Some(z_str)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
                    if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>()) {
                        temp_vertices_array.push(Vec3 { x, y, z })
                    }
                }
            }
            else if identifier == "vt" {
                if let (Some(x_str), Some(y_str)) = (line_iter.next(), line_iter.next()) {
                    if let (Ok(x), Ok(y)) = (x_str.parse::<f64>(), y_str.parse::<f64>()) {
                        temp_uvs_array.push(Vec2 { x, y })
                    }
                }
            }
            if identifier == "vn" {
                if let (Some(x_str), Some(y_str), Some(z_str)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
                    if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>()) {
                        temp_normals_array.push(Vec3 { x, y, z })
                    }
                }
            }
            if identifier == "f" {
                line_iter
            }
        }
    };
}
