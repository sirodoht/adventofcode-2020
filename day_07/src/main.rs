use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Rule {
    color: String,
    size: u32,
}

#[derive(Debug)]
struct Bag {
    color: String,
    rules: Vec<Rule>,
}

fn parse_rules(filename: &str) -> Vec<Bag> {
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

    // transform into a Vec<Bag>
    let mut bags: Vec<Bag> = vec![];
    for l in lines {
        let parts = l
            .split(" bags contain ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // get bag color
        let color = parts[0].to_string();
        let rules: Vec<Rule> = vec![];
        let mut new_bag = Bag { color, rules };

        let rules = parts[1].to_string();

        // end processing if rules is 0
        if rules == "no other bags." {
            bags.push(new_bag);
            continue;
        }

        // process non-zero bag rules
        let rules = rules.split(", ").collect::<Vec<&str>>();
        let mut new_rules: Vec<Rule> = vec![];
        for r in rules {
            let parts = r.split(' ').collect::<Vec<&str>>();
            let size = parts[0].parse().unwrap();
            let color = (parts[1].to_owned() + " " + parts[2]).to_string();
            let new_rule = Rule { color, size };
            new_rules.push(new_rule);
        }
        new_bag.rules = new_rules;

        bags.push(new_bag);
    }

    bags
}

fn process_shiny_gold_belonging(bags: &[Bag]) -> u32 {
    let mut rev_rules = HashMap::new();
    for b in bags {
        for r in &b.rules {
            rev_rules
                .entry(r.color.clone())
                .or_insert(vec![])
                .push(b.color.clone());
        }
    }

    let mut color_set = HashSet::new();
    let mut queue: Vec<&str> = vec!["shiny gold"];
    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if !rev_rules.contains_key(item) {
            continue;
        }
        for color in &rev_rules[item] {
            color_set.insert(color);
            queue.push(color);
        }
    }

    color_set.len() as u32
}

fn get_count(bags: &[Bag], color: &str) -> u32 {
    let mut acc: u32 = 0;
    for b in bags {
        if b.color == color {
            if b.rules.is_empty() {
                return 1;
            } else {
                for r in &b.rules {
                    acc += r.size * get_count(bags, &r.color);
                }
            }
            break;
        }
    }

    acc + 1
}

fn process_shiny_gold_count(bags: &[Bag]) -> u32 {
    get_count(bags, "shiny gold") - 1
}

fn main() {
    let filename = String::from("./src/input");
    let bags = parse_rules(&filename);

    let result = process_shiny_gold_belonging(&bags);
    println!("==result==");
    println!("{}", result);

    let result = process_shiny_gold_count(&bags);
    println!("==result part two==");
    println!("{}", result);
}
