use std::env;
use std::process;
use std::path::{Path, PathBuf};
use rosa::splitter::RoSplitter;

const RAW_NOISE_FILE: &str = "raw_noise_bits.bin";
const RESTART_FILE: &str = "restart_bits.bin";
const NUM_ROS: usize = 16;

fn get_absolute_path(init_path: &String) -> Result<PathBuf, String> {
    // Convert path to PathBuf
    let file_path = PathBuf::from(init_path);

    // Check if it is a directory
    if !file_path.is_dir() {
        return Err(String::from("Provided path is not a directory"));
    }

    // Check if it is already absolute
    if file_path.is_absolute() {
        return Ok(file_path);
    }

    let final_path = match file_path.canonicalize() {
        Ok(fp) => fp,
        Err(_) => return Err(String::from("Failed to canonicalize path"))
    };

    Ok(final_path)
}

fn main () {
    // Collect the arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Convert first arg into PathBuf
    let path_to_files = match get_absolute_path(&args[1]) {
        Ok(path) => path,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Create paths to the binary files
    let raw_noise_file = path_to_files.join(RAW_NOISE_FILE);
    let _restart_file = path_to_files.join(RESTART_FILE);

    let ro_split = match RoSplitter::new(raw_noise_file, NUM_ROS) {
        Ok(splitter) => splitter,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}
