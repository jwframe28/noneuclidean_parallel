extern crate itertools;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use rayon::prelude::*; // Import the traits from rayon

// Your existing check_triangle_inequality() function remains the same
fn check_triangle_inequality(n1: f64, n2: f64, n3:f64) -> bool {
    if n1 + n2 <= n3 || n1 + n3 <= n2 || n2 + n3 <= n1 {
        return false;
    } 
    true
}



fn main() -> std::io::Result<()> {
    // Your existing code for reading from a file and parsing the arguments
    // Retrieve the command line argument and convert it to a string
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name as a command-line argument.");
        return Ok(());
    }
    let filename = &args[1];

    // Read file into a String
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Vec<f64> = contents
        .trim()  // Remove any leading/trailing whitespace or new lines
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    let n = data.len() as i32;

    // Generate combinations using itertools
    let indices: Vec<i32> = (1..=data.len() as i32).collect();
    let combinations = indices.into_iter().combinations(3);

    let failed_combinations: Vec<Vec<i32>> = combinations
        .par_bridge()  // Convert to a parallel iterator
        .filter(|combo| !check_triangle_inequality(
            data[combo[0] as usize - 1],
            data[combo[1] as usize - 1],
            data[combo[2] as usize - 1]
        ))
        .collect();

    // Print the failed combinations
    for combo in &failed_combinations {
        println!("{},{},{}", combo[0], combo[1], combo[2]);
    }

    Ok(())
}
