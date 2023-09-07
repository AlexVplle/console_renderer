use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "console_renderer", author = "AlexVplle", version = "1.0", about = "OBJ file console renderer")]
pub struct Args {
    #[arg(short, long)]
    pub file_path: String,
}
