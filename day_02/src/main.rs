use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
enum ParseMode {
    Floor,
    Ceil,
    Letter,
    Password,
}

#[derive(Debug)]
struct Entry {
    floor: u32,
    ceil: u32,
    letter: char,
    password: String,
}

fn read_entries(filename: String) -> Vec<Entry> {
    // open file
    let path = Path::new(&filename);
    let path_display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", path_display, why),
        Ok(file) => file,
    };

    // read file contents
    let mut content = String::new();
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("could not read {}: {}", path_display, why)
    }

    // load data
    let mut entries: Vec<Entry> = vec![];
    let mut floor_str = String::new();
    let mut floor: u32 = 0;
    let mut ceil_str = String::new();
    let mut ceil: u32 = 0;
    let mut letter = '0';
    let mut password: String = String::new();
    let mut parse_mode = ParseMode::Floor;
    for value in content.chars() {
        match parse_mode {
            ParseMode::Floor => {
                if value == '-' {
                    floor = floor_str.parse().unwrap();
                    parse_mode = ParseMode::Ceil;
                    floor_str = String::new();
                } else {
                    floor_str.push(value);
                }
                continue;
            }

            ParseMode::Ceil => {
                if value == ' ' {
                    ceil = ceil_str.parse().unwrap();
                    parse_mode = ParseMode::Letter;
                    ceil_str = String::new();
                } else {
                    ceil_str.push(value);
                }
                continue;
            }

            ParseMode::Letter => {
                if value == ':' {
                    parse_mode = ParseMode::Password;
                } else {
                    letter = value;
                }
                continue;
            }

            ParseMode::Password => {
                if value == ' ' {
                    continue;
                }

                if value == '\n' {
                    let new_entry = Entry {
                        floor,
                        ceil,
                        letter,
                        password: password.clone(),
                    };
                    entries.push(new_entry);
                    password = String::new();
                    parse_mode = ParseMode::Floor;
                } else {
                    password.push(value);
                }
            }
        }
    }

    entries
}

fn main() {
    let filename = String::from("./src/input");
    let entries = read_entries(filename);
    println!("entries: {:?}", entries);

    // calculate part 1
    let mut valid_counter = 0;
    for entry in &entries {
        // measure how many times entry.letter appears in entry.password
        let mut letter_counter = 0;
        for c in entry.password.chars() {
            if entry.letter == c {
                letter_counter += 1;
            }
        }
        // if letter_counter is between floor and ceil inclusive, it's valid
        if letter_counter >= entry.floor && letter_counter <= entry.ceil {
            valid_counter += 1;
        }
    }

    println!("==result==");
    println!("{}", valid_counter);

    // calculate part 2
    let mut valid_counter = 0;
    for entry in &entries {
        // find letters on 1-indexed password string
        let pos_a: usize = entry.floor as usize - 1;
        let letter_a: char = entry.password.chars().nth(pos_a).unwrap();
        let pos_b: usize = entry.ceil as usize - 1;
        let letter_b: char = entry.password.chars().nth(pos_b).unwrap();

        // entry.letter must appear exactly once in those two positions
        if entry.letter != letter_a && entry.letter != letter_b {
            continue;
        }
        if entry.letter == letter_a && entry.letter == letter_b {
            continue;
        }
        valid_counter += 1;
    }

    println!("==result part two==");
    println!("{}", valid_counter);
}
