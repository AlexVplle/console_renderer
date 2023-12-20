use nalgebra::Vector3;
use std::{fs, path::Path, str::SplitWhitespace, usize};

pub fn load_obj(file_path: String) -> Result<Vec<Vector3<f64>>, String> {
    let mut temp_vertices_array: Vec<Vector3<f64>> = vec![];

    let mut vertices_array: Vec<Vector3<f64>> = vec![];

    let path: &Path = Path::new(&file_path);

    let is_valid_file: bool = if let Some(extension) = path.extension() {
        extension == "obj"
    } else {
        false
    };

    if !is_valid_file {
        return Err(String::from("Your file is not an OBJ file"));
    }

    let content: String = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(_) => return Err(String::from("Problem to read the file")),
    };

    for line in content.lines() {
        let mut line_iter: SplitWhitespace = line.split_whitespace();
        if let Some(identifier) = line_iter.next() {
            match identifier {
                "v" => {
                    if let (Some(x_str), Some(y_str), Some(z_str)) =
                        (line_iter.next(), line_iter.next(), line_iter.next())
                    {
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            x_str.parse::<f64>(),
                            y_str.parse::<f64>(),
                            z_str.parse::<f64>(),
                        ) {
                            temp_vertices_array.push(Vector3::new(x, y, z))
                        }
                    }
                }
                "f" => {
                    for point in line_iter {
                        let mut split = point.split("/");
                        if let Some(vertice_str) = split.next() {
                            if let Ok(vertice) = vertice_str.parse::<usize>() {
                                vertices_array.push(temp_vertices_array[vertice - 1].clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(vertices_array)
}
