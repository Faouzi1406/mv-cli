mod file_tree;
use file_tree::Directory;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    move_dir: String,
}

fn main() {
    let mut directory = Directory::new(Args::parse().dir);
    directory.map_directory(Args::parse().dir);
    
    
    // list all directory and its directory and recursive
    directory.list();
}
