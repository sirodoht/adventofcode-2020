use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

type RuleMapContains = HashMap<String, Vec<(String, u32)>>;
type RuleMapBelongs = HashMap<String, Vec<String>>;

fn parse_rules(filename: &str) -> (RuleMapContains, RuleMapBelongs) {
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
    let mut lines: Vec<&str> = content.split('\n').collect();
    lines.pop(); // pop last empty line

    // transform into hashmaps
    let mut rule_map_contains: RuleMapContains = HashMap::new();
    let mut rule_map_belongs: RuleMapBelongs = HashMap::new();
    for l in lines {
        let parts: Vec<&str> = l.split(" bags contain ").collect();
        let color = parts[0].to_string();
        let rules = parts[1];

        if rules == "no other bags." {
            rule_map_contains.insert(color, vec![]);
            continue;
        }

        let rules = rules.split(", ").collect::<Vec<&str>>();
        for r in rules {
            // build rule_map_contains
            let r_parts = r.split(' ').collect::<Vec<&str>>();
            let r_size: u32 = r_parts[0].parse().unwrap();
            let r_color = (r_parts[1].to_owned() + " " + r_parts[2]).to_string();
            let new_rule = (r_color.clone(), r_size);
            rule_map_contains.entry(color.clone()).or_insert(vec![]).push(new_rule);

            // build rule_map_belongs
            rule_map_belongs.entry(r_color.clone()).or_insert(vec![]).push(color.clone());
        }
    }
    
    (rule_map_contains, rule_map_belongs)
}

fn process_shiny_gold_belonging(rule_map: &RuleMapBelongs) -> u32 {
    let mut color_set = HashSet::new();
    let mut queue: Vec<&str> = vec!["shiny gold"];
    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if !rule_map.contains_key(item) {
            continue;
        }
        for color in &rule_map[item] {
            color_set.insert(color);
            queue.push(color);
        }
    }

    color_set.len() as u32
}

fn get_count(rule_map: &RuleMapContains, color: &str) -> u32 {
    let mut count: u32 = 0;
    for (key, value) in rule_map {
        if key == color {
            if value.is_empty() {
                return 1;
            } else {
                for (inner_color, inner_size) in value {
                    count += inner_size * get_count(rule_map, inner_color);
                }
            }
            break;
        }
    }

    count + 1
}

fn process_shiny_gold_count(rules: &RuleMapContains) -> u32 {
    get_count(rules, "shiny gold") - 1
}

fn main() {
    let filename = String::from("./src/input");
    let (rule_map_contains, rule_map_belongs) = parse_rules(&filename);

    let result = process_shiny_gold_belonging(&rule_map_belongs);
    println!("==result==");
    println!("{}", result);

    let result = process_shiny_gold_count(&rule_map_contains);
    println!("==result part two==");
    println!("{}", result);
}
