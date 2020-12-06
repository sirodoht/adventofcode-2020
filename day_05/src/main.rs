use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_boarding_passes(filename: String) -> Vec<String> {
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

    // process into vec of strings
    let mut lines: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
    lines.pop(); // discard last new line

    lines
}

fn calculate_seat(code: String) -> u32 {
    let (row, col) = code.split_at(7);

    // calculate seat row
    let mut upper: u32 = 127;
    let mut lower: u32 = 0;
    for s in row.chars() {
        let a: u32 = upper - lower;
        let b: u32 = a / 2;
        if s == 'F' {
            upper = lower + b;
        } else if s == 'B' {
            lower = lower + b + 1
        } else {
            panic!("unknown row character");
        }
    }
    assert_eq!(lower, upper);

    // calculate seat col
    let mut left: u32 = 0;
    let mut right: u32 = 7;
    for s in col.chars() {
        let a: u32 = right - left;
        let b: u32 = a / 2;
        if s == 'L' {
            right = left + b;
        } else if s == 'R' {
            left = left + b + 1;
        } else {
            panic!("unknown col character");
        }
    }
    assert_eq!(left, right);

    lower * 8 + left
}

fn main() {
    let filename = String::from("./src/input");
    let passes = read_boarding_passes(filename);

    // part one
    let mut seat_ids: Vec<u32> = vec![];
    let mut highest_seat_id = 0;
    for p in passes {
        let seat_id = calculate_seat(p);
        seat_ids.push(seat_id);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("==result==");
    println!("{}", highest_seat_id);

    // part two
    let mut missing_seat: u32 = 0;
    seat_ids.sort_unstable();
    let mut prev: u32 = *seat_ids.first().unwrap();
    for (index, value) in seat_ids.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if *value != prev + 1 {
            missing_seat = value - 1;
            break;
        }
        prev = *value;
    }

    println!("==result part two==");
    println!("{}", missing_seat);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_seat() {
        assert_eq!(super::calculate_seat(String::from("FBFBBFFRLR")), 357);
        assert_eq!(super::calculate_seat(String::from("BFFFBBFRRR")), 567);
        assert_eq!(super::calculate_seat(String::from("FFFBBBFRRR")), 119);
        assert_eq!(super::calculate_seat(String::from("BBFFBBFRLL")), 820);
    }
}
