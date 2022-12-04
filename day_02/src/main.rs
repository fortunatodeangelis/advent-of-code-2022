use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    // Get the file
    let path = "data/input.txt";

    // Open the file
    let input = File::open(path)?;

    // Create a buffered reader
    let buffered = BufReader::new(input);

    let mut elfs_rounds: Vec<i32> = Vec::new();
    let mut your_rounds: Vec<i32> = Vec::new();

    // For part 2
    let mut your_pilot_rounds: Vec<i32> = Vec::new();
    let mut elfs_pilot_rounds: Vec<i32> = Vec::new();

    // Iterate over the lines
    for line in buffered.lines() {
        let line = line?;
        let round: Vec<&str> = line.split(" ").collect();
        let mut your_score = 0;
        let mut elfs_score = 0;
        let mut your_pilot_score = 0;
        let mut elfs_pilot_score = 0;

        // Convert points
        elfs_score += convert_points(round[0]);
        your_score += convert_points(round[1]);

        // For part 2
        elfs_pilot_score += elfs_score;
        your_pilot_score += pilot_score(elfs_score, round[1]);

        println!("your score: {}", your_pilot_score);

        let result = score_round(elfs_score, your_score);

        let result_pilot = score_round(elfs_pilot_score, your_pilot_score);

        // Check round result
        if result == "1" {
            elfs_score +=6;
        }else if result == "2" {
            your_score +=6;
        } else{
            elfs_score +=3;
            your_score +=3;
        }

        // Check round result for part 2
        if result_pilot == "1" {
            elfs_pilot_score +=6;
        }else if result_pilot == "2" {
            your_pilot_score +=6;
        } else{
            elfs_pilot_score +=3;
            your_pilot_score +=3;
        }

        your_rounds.push(your_score);
        elfs_rounds.push(elfs_score);

        // For part 2
        elfs_pilot_rounds.push(elfs_pilot_score);
        your_pilot_rounds.push(your_pilot_score);
    }

    // Sum all scores of round
    let elfs_total_score: i32 = elfs_rounds.iter().sum();
    let yours_total_score: i32 = your_rounds.iter().sum();

    // For part 2
    let your_pilot_total_score: i32 = your_pilot_rounds.iter().sum();
    let elfs_pilot_total_score: i32 = elfs_pilot_rounds.iter().sum();

    println!("Secondo la guida strategica hai totalizzato {} punti mentre l'elfo ha totalizzato {}", yours_total_score, elfs_total_score);

    // For part 2
    println!("Secondo la guida strategica dove controlli il risultato hai totalizzato {} punti mentre l'elfo ha totalizzato {}", your_pilot_total_score, elfs_pilot_total_score);

    Ok(())
}


fn score_round(elfs_score:i32, your_score:i32) -> &'static str {
    let r = if (elfs_score) % 3 == (your_score - 1) {
        "2" // you win
    } else if elfs_score == your_score {
        "x" // draw
    } else {
        "1" // elf win
    };
    r
}

fn convert_points(char: &str) -> i32 {
    let r = match char {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };
    return r;
}

fn pilot_score(elfs_score:i32, char:&str) -> i32 {
    let r = if char == "Y" {
        elfs_score  
    } else if char == "Z" {
        if elfs_score == 3 {
            1
        }else {
            elfs_score + 1
        }
    } else {
        if elfs_score == 1 {
            3
        }else {
            elfs_score - 1
        }
    };
    r
}