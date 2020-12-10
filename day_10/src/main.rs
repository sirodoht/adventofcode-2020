use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_numbers(filename: &str) -> Vec<u64> {
    // open file
    let path = Path::new(filename);
    let path_display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", path_display, why),
        Ok(file) => file,
    };

    // read file contents into a string
    let mut content = String::new();
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("could not read {}: {}", path_display, why)
    }

    // transform into a vec of u64
    let mut lines: Vec<&str> = content.split('\n').collect();
    lines.pop(); // pop last empty line
    let numbers: Vec<u64> = lines.iter().map(|x| x.parse().unwrap()).collect();

    numbers
}

fn main() {
    let filename = String::from("./src/input");
    let numbers = read_numbers(&filename);

    // part one
    let mut differences = HashMap::new();
    let mut sorted = numbers;
    sorted.sort_unstable();
    sorted.push(sorted.iter().max().unwrap() + 3); // add built-in adapter joltage
    let mut prev = 0;
    for n in &sorted {
        *differences.entry(n - prev).or_insert(0) += 1;
        prev = *n;
    }

    // NOTE: there are no differences of 2!
    assert!(differences.get(&2).is_none());

    let result = differences[&1] * differences[&3];
    println!("==result==");
    println!("{}", result);

    // part two
    // calculate all possible combinations by counting how many consecutive 1s
    let mut product: u64 = 1;
    let mut prev = 0;
    let mut consecutive = 0;
    for n in &sorted {
        if n - prev == 1 {
            consecutive += 1;
        } else {
            // depending on the consecutive 1s, multiply final product by
            // all the possible combinations. There is no case of more
            // than 4 consecutive 1s.
            if consecutive == 4 {
                product *= 7;
            } else if consecutive == 3 {
                product *= 4;
            } else if consecutive == 2 {
                product *= 2;
            }

            consecutive = 0;
        }
        prev = *n;
    }
    println!("==result part two==");
    println!("{}", product);
}
