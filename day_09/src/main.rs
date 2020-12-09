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

/// is_sum_batch returns true if a subgroup of the numbers that belong in batch
/// are equal the sum given
fn is_sum_batch(sum: u64, batch: &[u64]) -> bool {
    for (outer_index, outer_value) in batch.iter().enumerate() {
        let reference = outer_value;
        for (inner_index, inner_value) in batch.iter().enumerate() {
            if inner_index < outer_index {
                // those have already been checked
                continue;
            }
            if reference == inner_value {
                continue;
            }
            if reference + inner_value == sum {
                return true;
            }
        }
    }

    false
}

fn main() {
    let filename = String::from("./src/input");
    let numbers = read_numbers(&filename);

    // part one
    let batch_size = 25;
    let mut invalid_sum: u64 = 0;
    for (index, n) in numbers.iter().enumerate() {
        if index < batch_size {
            // ignore the first 5 or 25 since they are the preable
            continue;
        }
        let prev_batch = &numbers[index - batch_size..index];
        if !is_sum_batch(*n, &prev_batch) {
            invalid_sum = *n;
        }
    }

    println!("==result==");
    println!("{}", invalid_sum);

    // part two
    let mut running_sum: u64 = 0;
    let mut sum_items: Vec<u64> = vec![];
    'outer: for starting_index in 0..numbers.len() {
        for &n in &numbers[starting_index..] {
            running_sum += n;
            sum_items.push(n);
            if n != invalid_sum && running_sum == invalid_sum {
                // sum found, break outer loop
                break 'outer;
            } else if running_sum > invalid_sum {
                // running_sum is over our target number, break inner loop
                running_sum = 0;
                sum_items.clear();
                break;
            }
        }
    }

    // add up smallest and larger
    let smallest = sum_items.iter().min().unwrap();
    let largest = sum_items.iter().max().unwrap();
    let encryption_weakness = smallest + largest;

    println!("==result part two==");
    println!("{}", encryption_weakness);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_sum_batch() {
        assert!(super::is_sum_batch(40, &[35, 20, 15, 25, 47]));
        assert!(super::is_sum_batch(62, &[20, 15, 25, 47, 40]));
        assert!(super::is_sum_batch(55, &[15, 25, 47, 40, 62]));
        assert!(!super::is_sum_batch(50, &[15, 25, 47, 40, 62]));
    }
}
