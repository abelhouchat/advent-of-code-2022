use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn ascii_to_priority(letter: &char) -> u32 {
    let ascii_code = *letter as u32;

    let priority: u32 = if letter.is_ascii_lowercase() {
        ascii_code - 96
    } else {
        ascii_code - 38
    };

    return priority;
}

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // PART ONE
    ////////////////////////////////////////////////////////////////////////////////

    let filename = "input.txt";

    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut priority_sum: u32 = 0;
    for line in reader.lines() {
        let rucksack = match line {
            Err(why) => panic!("Input is broked: {}", why),
            Ok(line) => line,
        };

        let sack_size = rucksack.len() / 2;
        let sack_1 = &rucksack[..sack_size];
        let sack_2 = &rucksack[sack_size..];

        let mut found_common = false;

        let mut sack_1_iter = sack_1.chars();
        while !found_common {
            let item_1 = sack_1_iter.next().expect("No remaining items");
            let priority_1 = ascii_to_priority(&item_1);
            for item_2 in sack_2.chars() {
                let priority_2 = ascii_to_priority(&item_2);
                if priority_1 == priority_2 {
                    priority_sum += priority_1;
                    found_common = true;
                    break;
                }
            }
        }
    }

    println!("Common item sum is {priority_sum}");

    ////////////////////////////////////////////////////////////////////////////////
    // PART TWO
    ////////////////////////////////////////////////////////////////////////////////

    let filename = "input.txt";

    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut badge_sum: u32 = 0;

    let mut lines = reader.lines();
    loop {
        let sack_1 = match lines.next() {
            None => break,
            Some(string) => string.expect("Problem reading first sack"),
        };
        let sack_2 = match lines.next() {
            None => panic!("Found a group with less than three sacks"),
            Some(string) => string.expect("Problem reading second sack"),
        };
        let sack_3 = match lines.next() {
            None => panic!("Found a group with less than three sacks"),
            Some(string) => string.expect("Problem reading third sack"),
        };

        let mut found_badge = false;
        let mut sack_1_iter = sack_1.chars();
        while !found_badge {
            let item_1 = sack_1_iter.next().expect("No remaining items");
            if sack_2.contains(item_1) && sack_3.contains(item_1) {
                badge_sum += ascii_to_priority(&item_1);
                found_badge = true;
            }
        }
    }

    println!("Badge sum is {badge_sum}");
}
