use std::fs::File;
use std::io::Read;
use std::path::Path;

fn print_seats(seats: &[String]) {
    for line in seats {
        println!("{}", line);
    }
}

fn read_seats(filename: &str) -> Vec<String> {
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

    // transform into a vec of strings
    let mut lines: Vec<String> = content.split('\n').map(|x| x.to_string()).collect();
    lines.pop(); // pop last empty line

    lines
}

fn get_neighbours(seats: &[String], row: usize, col: usize) -> Vec<(usize, usize)> {
    let row_max = seats.len();
    let col_max = seats.first().unwrap().len();
    let mut neighbours = vec![];

    // top left
    if row >= 1 && col >= 1 {
        neighbours.push((row - 1, col - 1));
    }

    // top mid
    if row >= 1 {
        neighbours.push((row - 1, col));
    }

    // top right
    if row >= 1 && col + 1 < col_max {
        neighbours.push((row - 1, col + 1));
    }

    // mid left
    if col >= 1 {
        neighbours.push((row, col - 1));
    }

    // mid right
    if col + 1 < col_max {
        neighbours.push((row, col + 1));
    }

    // bottom left
    if row + 1 < row_max && col >= 1 {
        neighbours.push((row + 1, col - 1));
    }

    // bottom mid
    if row + 1 < row_max {
        neighbours.push((row + 1, col));
    }

    // bottom right
    if row + 1 < row_max && col + 1 < col_max {
        neighbours.push((row + 1, col + 1));
    }

    neighbours
}

fn get_evolved_seat(seats: &[String], row: usize, col: usize) -> char {
    let neighbours = get_neighbours(&seats, row, col);
    let seat: char = seats[row].chars().nth(col).unwrap();

    //println!("neighbours: {:?}", neighbours);

    if seat == 'L' {
        for (row, col) in neighbours {
            let neighbour = seats[row].chars().nth(col).unwrap();
            if neighbour == '#' {
                return 'L';
            }
        }
        return '#';
    } else if seat == '#' {
        let mut occupied_count = 0;
        for (row, col) in neighbours {
            let neighbour = seats[row].chars().nth(col).unwrap();
            if neighbour == '#' {
                occupied_count += 1;
            }
        }
        if occupied_count >= 4 {
            return 'L';
        }
    }

    seat
}

fn evolve(seats: &[String]) -> Vec<String> {
    let mut evolved_matrix: Vec<String> = vec![];
    for (row, line) in seats.iter().enumerate() {
        let mut evolved_line = String::new();
        for (col, _) in line.chars().enumerate() {
            let evolved_seat = get_evolved_seat(&seats, row, col);
            evolved_line.push(evolved_seat);
        }
        evolved_matrix.push(evolved_line);
    }

    evolved_matrix
}

fn are_equal(seats_a: &[String], seats_b: &[String]) -> bool {
    for (index, line) in seats_a.iter().enumerate() {
        if *line != seats_b[index] {
            return false;
        }
    }

    true
}

fn count_occupied(seats: &[String]) -> u32 {
    let mut count = 0;
    for line in seats {
        for c in line.chars() {
            if c == '#' {
                count += 1;
            }
        }
    }

    count
}

fn evolve_b(seats: &[String]) -> Vec<String> {
    let mut evolved_matrix: Vec<String> = vec![];
    for (row, line) in seats.iter().enumerate() {
        let mut evolved_line = String::new();
        for (col, _) in line.chars().enumerate() {
            let evolved_seat = get_evolved_seat_b(&seats, row, col);
            evolved_line.push(evolved_seat);
        }
        evolved_matrix.push(evolved_line);
    }

    evolved_matrix
}

fn get_evolved_seat_b(seats: &[String], row: usize, col: usize) -> char {
    let seat: char = seats[row].chars().nth(col).unwrap();
    let occupied_count = get_visible_count(seats, row, col);

    if seat == 'L' && occupied_count == 0 {
        return '#';
    } else if seat == '#' && occupied_count >= 5 {
        return 'L';
    }

    seat
}

fn get_visible_count(seats: &[String], row: usize, col: usize) -> u32 {
    let mut visible_count = 0;
    let row_max = seats.len() as i64;
    let col_max = seats.first().unwrap().len() as i64;

    for row_dimension in -1..=1 {
        for col_dimension in -1..=1 {
            if row_dimension == 0 && col_dimension == 0 {
                continue;
            }
            let mut to_check_row = row as i64;
            let mut to_check_col = col as i64;

            loop {
                to_check_row += row_dimension;
                to_check_col += col_dimension;
                if to_check_row < 0
                    || to_check_row >= row_max
                    || to_check_col < 0
                    || to_check_col >= col_max
                {
                    break;
                }

                let seat_current = seats[to_check_row as usize]
                    .chars()
                    .nth(to_check_col as usize)
                    .unwrap();
                if seat_current == '#' {
                    visible_count += 1;
                    break;
                } else if seat_current == 'L' {
                    break;
                }
            }
        }
    }

    visible_count
}

fn main() {
    let filename = String::from("./src/input");
    let seats = read_seats(&filename);
    print_seats(&seats);

    // part one
    let mut prev_iteration = seats.clone();
    let mut evolved = evolve(&prev_iteration);
    let mut count_iterations = 1;
    while !are_equal(&evolved, &prev_iteration) {
        prev_iteration = evolved;
        evolved = evolve(&prev_iteration);
        count_iterations += 1;
    }
    println!("count_iterations: {}", count_iterations);

    let result = count_occupied(&evolved);
    println!("==result==");
    println!("{}", result);

    // part two
    let mut prev_iteration = seats.clone();
    let mut evolved = evolve_b(&seats);
    let mut count_iterations = 1;
    while !are_equal(&evolved, &prev_iteration) {
        prev_iteration = evolved;
        evolved = evolve_b(&prev_iteration);
        count_iterations += 1;
        //print_seats(&evolved);
    }
    println!("count_iterations: {}", count_iterations);

    let result = count_occupied(&evolved);
    println!("==result part two==");
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_evolve_first_iteration() {
        let seats = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        let expected = vec![
            "#.##.##.##",
            "#######.##",
            "#.#.#..#..",
            "####.##.##",
            "#.##.##.##",
            "#.#####.##",
            "..#.#.....",
            "##########",
            "#.######.#",
            "#.#####.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert!(super::are_equal(&evolved, &expected));
    }

    #[test]
    fn test_evolve_second_iteration() {
        let seats = vec![
            "#.##.##.##",
            "#######.##",
            "#.#.#..#..",
            "####.##.##",
            "#.##.##.##",
            "#.#####.##",
            "..#.#.....",
            "##########",
            "#.######.#",
            "#.#####.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        let expected = vec![
            "#.LL.L#.##",
            "#LLLLLL.L#",
            "L.L.L..L..",
            "#LLL.LL.L#",
            "#.LL.LL.LL",
            "#.LLLL#.##",
            "..L.L.....",
            "#LLLLLLLL#",
            "#.LLLLLL.L",
            "#.#LLLL.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert!(super::are_equal(&evolved, &expected));
    }

    #[test]
    fn test_evolve_third_iteration() {
        let seats = vec![
            "#.LL.L#.##",
            "#LLLLLL.L#",
            "L.L.L..L..",
            "#LLL.LL.L#",
            "#.LL.LL.LL",
            "#.LLLL#.##",
            "..L.L.....",
            "#LLLLLLLL#",
            "#.LLLLLL.L",
            "#.#LLLL.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        let expected = vec![
            "#.##.L#.##",
            "#L###LL.L#",
            "L.#.#..#..",
            "#L##.##.L#",
            "#.##.LL.LL",
            "#.###L#.##",
            "..#.#.....",
            "#L######L#",
            "#.LL###L.L",
            "#.#L###.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert!(super::are_equal(&evolved, &expected));
    }

    #[test]
    fn test_evolve_fourth_iteration() {
        let seats = vec![
            "#.##.L#.##",
            "#L###LL.L#",
            "L.#.#..#..",
            "#L##.##.L#",
            "#.##.LL.LL",
            "#.###L#.##",
            "..#.#.....",
            "#L######L#",
            "#.LL###L.L",
            "#.#L###.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        let expected = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.L.L..#..",
            "#LLL.##.L#",
            "#.LL.LL.LL",
            "#.LL#L#.##",
            "..L.L.....",
            "#L#LLLL#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert!(super::are_equal(&evolved, &expected));
    }

    #[test]
    fn test_evolve_fifth_iteration() {
        let seats = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.L.L..#..",
            "#LLL.##.L#",
            "#.LL.LL.LL",
            "#.LL#L#.##",
            "..L.L.....",
            "#L#LLLL#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        let expected = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.#.L..#..",
            "#L##.##.L#",
            "#.#L.LL.LL",
            "#.#L#L#.##",
            "..L.L.....",
            "#L#L##L#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert!(super::are_equal(&evolved, &expected));
    }

    #[test]
    fn test_evolve_six_iteration() {
        let seats = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.#.L..#..",
            "#L##.##.L#",
            "#.#L.LL.LL",
            "#.#L#L#.##",
            "..L.L.....",
            "#L#L##L#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let evolved = super::evolve(&seats);

        assert!(super::are_equal(&evolved, &seats));
    }

    #[test]
    fn test_get_visible_count() {
        let seats = vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        assert_eq!(super::get_visible_count(&seats, 4, 3), 8);
    }

    #[test]
    fn test_get_visible_count_small() {
        let seats = vec![".............", ".L.L.#.#.#.#.", "............."]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        assert_eq!(super::get_visible_count(&seats, 2, 2), 0);
    }
}
