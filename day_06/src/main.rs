use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_content(filename: &str) -> String {
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

    content
}

fn read_forms_flatten(filename: &str) -> Vec<String> {
    let content = read_content(filename);
    let lines: Vec<&str> = content.split("\n\n").collect();
    let lines: Vec<String> = lines.iter().map(|s| s.trim().replace('\n', "")).collect();

    lines
}

fn read_forms(filename: &str) -> Vec<Vec<String>> {
    let content = read_content(filename);
    let lines: Vec<&str> = content.split('\n').collect();

    let mut forms: Vec<Vec<String>> = vec![];
    let mut new_form: Vec<String> = vec![];
    for l in lines {
        if l == "" {
            forms.push(new_form);
            new_form = vec![];
        } else {
            new_form.push(l.to_string());
        }
    }

    forms
}

fn main() {
    let filename = String::from("./src/input");
    let forms = read_forms_flatten(&filename);

    let mut questions: Vec<HashSet<char>> = vec![];
    for f in &forms {
        let mut hs = HashSet::new();
        for c in f.chars() {
            hs.insert(c);
        }
        questions.push(hs);
    }

    let sum: u32 = questions
        .iter()
        .flat_map(|x| x.iter())
        .fold(0, |acc, _| acc + 1);
    println!("==result==");
    println!("{}", sum);

    // part two
    let mut everyone_count = 0;
    let forms = read_forms(&filename);
    for group in &forms {
        let mut group_mapping = HashMap::new();
        for person in group {
            for answer in person.chars() {
                *group_mapping.entry(answer).or_insert(0) += 1;
            }
        }

        for (_, value) in group_mapping {
            if group.len() == value as usize {
                everyone_count += 1;
            }
        }
    }

    println!("==result part two ==");
    println!("{}", everyone_count);
}
