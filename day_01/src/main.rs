use std::fs::File;
use std::io::{BufReader, BufRead, Error};


fn main() -> Result<(), Error> {

    let mut elf: Vec<i32> = Vec::new();
    // Get the file
    let path = "data/input.txt";

    // Open the file
    let input = File::open(path)?;

    // Create a buffered reader
    let buffered = BufReader::new(input);

    // Create a index for vector
    let mut index = 0;
    
    // Iterate over the lines
    for line in buffered.lines() {
        let line = line?;

        // Check if exist index in vector
        if elf.len() <= index {
            elf.push(0);
        }
      
        if line.len() > 0 {
            elf[index] += line.parse::<i32>().unwrap();
        }else {
            index += 1;
        }
    }

    // Check the max value in vect
    let mut max = elf.iter().max().unwrap();
    
    // Check the index of max value
    let mut index_max = elf.iter().position(|&r| r == *max).unwrap();

    println!("L'elfo {} ha totalizzato il maggior numero di calorie: {}", index_max, max);

    // Sum all calories
    let mut total_calories: i32= *max;

    // Remove max value
    elf.remove(index_max);

    // Remove index with max value in element for three times
    for _ in 0..2 {
        // Check the max value in vect
        max = elf.iter().max().unwrap();
        
        // Check the index of max value
        index_max = elf.iter().position(|&r| r == *max).unwrap();

        // add max value to total calories
        total_calories += *max;

        // Remove max value
        elf.remove(index_max);
    }

    println!("Gli ultimi elfi hano totalizzato {} calorie", total_calories);

    Ok(())
}