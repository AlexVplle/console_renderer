mod loader;
mod structures;

use clap::Parser;
use crossterm::terminal;

#[derive(Parser, Debug)]
#[command(name = "console_renderer", author = "AlexVplle", version = "1.0", about = "OBJ file console renderer")]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args : Args = Args::parse();
    let mut vertices_array = vec![];
    let mut uvs_array = vec![];
    let mut normals_array = vec![];
    if !loader::load_obj(args.file_path ,&mut vertices_array, &mut uvs_array, &mut normals_array) {
        return;
    }
    dbg!(terminal::size().unwrap());
}

