use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    // Get the file
    let path = "data/input.txt";

    // Open the file
    let input = File::open(path)?;

    // Create a buffered reader
    let buffered = BufReader::new(input);

    let mut sum_priority = 0;

    // Chunk buffered lines in groups of 3 lines
    let lines = buffered.lines().collect::<Vec<_>>();
    lines.chunks(3).for_each(|chunk| {

        // Iterate characters in the first line
        for i in 0..chunk[0].as_ref().unwrap().len() {
            // Get the character
            let c = chunk[0].as_ref().unwrap().chars().nth(i).unwrap();

            // Check if the character is in the other two lines
            if chunk[1].as_ref().unwrap().contains(c) && chunk[2].as_ref().unwrap().contains(c) {

                sum_priority += if c as i32 >= 97 {
                    c as i32 - 96
                } else {
                    c as i32 - 38
                };
                break;
            }
        }
    });

    println!("La somma delle priorità è {}", sum_priority);

    Ok(())
}