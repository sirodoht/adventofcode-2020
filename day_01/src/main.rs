use std::fs::File;
use std::path::Path;
use std::io::Read;

fn read_number_list(filename: String) -> Vec<u32> {
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

    // load data into Vec<u32>
    let mut all_numbers: Vec<u32> = vec![];
    let mut number_str = String::new();
    for c in content.chars() {
        //println!("reading c: {}", c);
        if c == '\n' {
            let number_u32: u32 = number_str.parse().unwrap();
            //println!("number_u32: {}", number_u32);
            all_numbers.push(number_u32);
            number_str = String::new();
        } else {
            number_str.push(c);
        }
        //println!("number_str: {}\n", number_str);
    }

    all_numbers
}

fn main() {
    let filename = String::from("./src/input");
    let all_numbers = read_number_list(filename);
    println!("all_numbers: {:?}", all_numbers);

    // find those two with sum of 2020
    let mut sum_entries = vec![0, 0];
    for (index_a, value_a) in all_numbers.iter().enumerate() {
        for (index_b, value_b) in all_numbers.iter().enumerate() {
            if index_b < index_a {
                continue;
            }
            if value_a + value_b == 2020 {
                println!("value_a: {}, value_b: {}", value_a, value_b);
                sum_entries[0] = *value_a;
                sum_entries[1] = *value_b;
            }
        }
    }

    // multiply sum entries - result of part 1
    let multiply_result = sum_entries[0] * sum_entries[1];
    println!("==result==");
    println!("{}", multiply_result);

    // find those three with sum of 2020
    let mut sum_entries = vec![0, 0, 0];
    for (index_a, value_a) in all_numbers.iter().enumerate() {
        for (index_b, value_b) in all_numbers.iter().enumerate() {
            for (index_c, value_c) in all_numbers.iter().enumerate() {
                if index_b < index_a || index_c < index_b {
                    continue;
                }
                if value_a + value_b + value_c == 2020 {
                    println!("value_a: {}, value_b: {}, value_c: {}", value_a, value_b, value_c);
                    sum_entries[0] = *value_a;
                    sum_entries[1] = *value_b;
                    sum_entries[2] = *value_c;
                }
            }
        }
    }

    // multiply sum entries - result of part 1
    let multiply_result = sum_entries[0] * sum_entries[1] * sum_entries[2];
    println!("==result part two==");
    println!("{}", multiply_result);
}
