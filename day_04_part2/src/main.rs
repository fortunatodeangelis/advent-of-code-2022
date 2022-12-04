use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    // Get the file
    let path = "data/input.txt";

    // Open the file
    let input = File::open(path)?;

    // Create a buffered reader
    let buffered = BufReader::new(input);

    let mut sum_range: i32 = 0;
    
    for line in buffered.lines() {
        let line = line?;
        // split line in two part by comma
        let couple: Vec<&str> = line.split(",").collect();
        let range_one: Vec<&str> = couple[0].split("-").collect();
        let range_two: Vec<&str> = couple[1].split("-").collect();

        // check if range of number in couple[0] superimposes range of number in couple[1]
        if range_one[1].parse::<i32>().unwrap() >= range_two[0].parse::<i32>().unwrap() && 
        range_one[0].parse::<i32>().unwrap() <= range_two[1].parse::<i32>().unwrap() {
                sum_range += 1;
        }
    }

    println!("Il numero di range che si sovramppongono è: {}", sum_range);

    Ok(())
}