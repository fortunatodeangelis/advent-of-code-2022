use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    // Get the file
    let path = "data/input.txt";

    // Open the file
    let input = File::open(path)?;

    // Create a buffered reader
    let buffered = BufReader::new(input);

    let mut sum_priority: i32 = 0;

    for line in buffered.lines() {
        let articles = line?;
        let n = articles.len();

        let compartment= articles.chars()
        .collect::<Vec<char>>()
        .chunks(n/2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

        // Extract same char in two compartment
        let mut element : String = String::new();
        for i in 0..compartment[0].len() {
            let article = compartment[0].chars().nth(i).unwrap().to_string();
            if compartment[1].contains(&article) {
                element.push_str(&article);
                // exit from loop
                break;
            }
        }
        let value_article = if element.chars().nth(0).unwrap() as i32 >= 97 {
            element.chars().nth(0).unwrap() as i32 - 96
        } else {
            element.chars().nth(0).unwrap() as i32 - 38
        };

        sum_priority += value_article;
        println!("La priorità per l'elemento {} è {}", element, value_article);
    }

    println!("La somma delle priorità è {}", sum_priority);

    Ok(())
}