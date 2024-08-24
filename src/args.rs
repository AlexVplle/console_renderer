use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, PartialOrd, Default)]
#[command(name = "console_renderer", author = "AlexVplle", version = "1.0", about = "OBJ file console renderer")]
pub struct Args {
    #[arg(short, long)]
    pub file_path: String,
}
