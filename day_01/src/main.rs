use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

    let mut calories_max: u64 = 0;
    let mut calories_current: u64 = 0;
    let mut elf_max: u32 = 1;
    let mut elf_current: u32 = 1;

    for line in reader.lines() {
        let calorie_str = match line {
            Err(why) => panic!("Input is broked: {}", why),
            Ok(line) => line,
        };
        if calorie_str == "" {
            if calories_current > calories_max {
                calories_max = calories_current;
                elf_max = elf_current;
            }

            println!("Elf {} has {} calories", elf_current, calories_current);

            calories_current = 0;
            elf_current += 1;
        } else {
            let calories: u64 = calorie_str.trim().parse().expect("Couldn't parse");
            calories_current += calories;
        }
    }

    println!("Elf {} has the most calories ({})", elf_max, calories_max);

    ////////////////////////////////////////////////////////////////////////////////
    // PART TWO
    ////////////////////////////////////////////////////////////////////////////////

    let filename = "input.txt";

    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut calories_max: [u64; 3] = [0, 0, 0];
    let mut calories_current: u64 = 0;
    let mut elf_max: [u32; 3] = [1, 1, 1];
    let mut elf_current: u32 = 1;

    for line in reader.lines() {
        let calorie_str = match line {
            Err(why) => panic!("Input is broked: {}", why),
            Ok(line) => line,
        };
        if calorie_str == "" {
            if calories_current > calories_max[0] {
                calories_max[2] = calories_max[1];
                calories_max[1] = calories_max[0];
                calories_max[0] = calories_current;
                elf_max[2] = elf_max[1];
                elf_max[1] = elf_max[0];
                elf_max[0] = elf_current;
            } else if calories_current > calories_max[1] {
                calories_max[2] = calories_max[1];
                calories_max[1] = calories_current;
                elf_max[2] = elf_max[1];
                elf_max[1] = elf_current;
            } else if calories_current > calories_max[2] {
                calories_max[2] = calories_current;
                elf_max[2] = elf_current
            }

            calories_current = 0;
            elf_current += 1;
        } else {
            let calories: u64 = calorie_str.trim().parse().expect("Couldn't parse");
            calories_current += calories;
        }
    }

    println!("Top three elves: {:?}", elf_max);
    println!("Top three calories: {:?}", calories_max);

    let calories_total: u64 = calories_max.iter().sum();
    println!("Total combined top three calories: {calories_total}");
}
