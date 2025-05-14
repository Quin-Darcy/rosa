use std::process;
use rosa::splitter::RoSplitter;


const BINARY_FILE: &str = "test.txt";
const NUM_ROS: usize = 16;

fn main () {
    let ro_split = match RoSplitter::new(BINARY_FILE, NUM_ROS) {
        Ok(splitter) => splitter,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}
