use std::{process, env};
use crate::handlers::path_exist::is_path_exist;


pub fn get_input_params() -> (String, String, u32) {
    let args: Vec<String> = env::args().collect();
    
    // Check length of arguments
    if args.len() != 4 {
        eprintln!("The number of arguments is not equal to 3");
        process::exit(1); 
    }
    
    let input_file_path: String  = args[1].clone();
    let output_file_path: String = args[2].clone();
    let quality = args[3].parse().expect("Quality argument must be a number");

    if !is_path_exist(&input_file_path) {
        eprintln!("Path didn't exist");
        process::exit(1);
    }

    // Checking if an argument can be a percentage
    if quality > 100 || quality < 1 {
        eprintln!("Quality argument must be a number < 100 and > 0");
        process::exit(1);
    }

    return (input_file_path, output_file_path, quality);
}