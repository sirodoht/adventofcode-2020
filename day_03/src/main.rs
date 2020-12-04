use std::fs::File;
use std::io::Read;
use std::path::Path;

fn print_map(map: &[Vec<bool>]) {
    for line in map {
        for c in line {
            if *c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read_map(filename: String) -> Vec<Vec<bool>> {
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

    // load data into boolean map
    let mut map: Vec<Vec<bool>> = vec![];
    let mut line: Vec<bool> = vec![];
    for value in content.chars() {
        if value == '.' {
            line.push(false);
        } else if value == '#' {
            line.push(true);
        } else if value == '\n' {
            map.push(line);
            line = vec![];
        } else {
            panic!("unknown character");
        }
    }

    map
}

fn get_tree_collisions(map: &[Vec<bool>], right: usize, down: usize) -> u32 {
    let mut current_col = 0;
    let mut current_row = 0;
    let mut trees_count = 0;
    let mut rows_left = map.len();

    while rows_left > 0 && current_row < map.len() {
        let line = &map[current_row];
        // if true, then we collided with a tree
        if line[current_col] {
            trees_count += 1;
        }

        // fake copying map to the right eternally with modulus
        current_col = (current_col + right) % line.len();

        current_row += down;

        rows_left -= 1;
    }

    trees_count
}

fn main() {
    let filename = String::from("./src/input");
    let map = read_map(filename);
    print_map(&map);

    // part one
    let result_part_one = get_tree_collisions(&map, 3, 1);
    println!("==result==");
    println!("{}", result_part_one);

    // part two
    let part_two_params = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let part_two_counts: Vec<u32> = part_two_params
        .iter()
        .map(|x| get_tree_collisions(&map, x[0], x[1]))
        .collect();
    println!("part_two_counts: {:?}", part_two_counts);
    let result_part_two: u32 = part_two_counts.iter().product();

    println!("==result part two==");
    println!("{}", result_part_two);
}
