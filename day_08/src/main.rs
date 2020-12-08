use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone)]
struct Instruction {
    cmd: String,
    op: char,
    val: u32,
}

fn parse_code(filename: &str) -> HashMap<u32, Instruction> {
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

    // transform into hashmap
    let mut code = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        // nop +0 -> ["nop", "+0"]
        let parts: Vec<&str> = line.split(' ').collect();

        let key = index as u32 + 1; // hashmap key is the line number
        let cmd = parts[0].to_string(); // eg. "nop"
        let (op, val) = parts[1].split_at(1); // "+0" -> ["+", "0"]
        let op: char = op.chars().next().unwrap(); // "+" -> '+'
        let val = val.parse().unwrap();
        let instr = Instruction { cmd, op, val };
        code.insert(key, instr);
    }

    code
}

fn process_code(code: &HashMap<u32, Instruction>) -> (i64, Vec<u32>, bool) {
    let mut acc: i64 = 0;
    let mut current: u32 = 1;
    let mut execution_set = HashSet::new();
    let mut sequence: Vec<u32> = vec![];
    let mut terminated = false;

    loop {
        if current - 1 == code.len().try_into().unwrap() {
            println!("terminated true, current: {}, acc: {}", current, acc);
            terminated = true;
            break;
        }

        if !code.contains_key(&current) {
            println!("line does not exist: {}, breaking", current);
            break;
        }

        if execution_set.contains(&current) {
            //println!("line already run: {} => {:?}", current, code[&current]);
            break;
        }

        execution_set.insert(current);
        sequence.push(current);
        if code[&current].cmd == "acc" {
            //println!("running now: {} => {:?}", current, code[&current]);
            if code[&current].op == '+' {
                acc += code[&current].val as i64;
            } else if code[&current].op == '-' {
                acc -= code[&current].val as i64;
            } else {
                panic!("unknown op: {}", code[&current].op);
            }
            current += 1;
        } else if code[&current].cmd == "jmp" {
            //println!("running now: {} => {:?}", current, code[&current]);
            if code[&current].op == '+' {
                current += code[&current].val;
            } else if code[&current].op == '-' {
                current -= code[&current].val;
            } else {
                panic!("unknown op: {}", code[&current].op);
            }
        } else if code[&current].cmd == "nop" {
            //println!("running now: {} => {:?}", current, code[&current]);
            current += 1;
        } else {
            panic!("uknown cmd: {}", code[&current].cmd);
        }
    }

    (acc, sequence, terminated)
}

fn main() {
    let filename = String::from("./src/input");
    let code = parse_code(&filename);
    //println!("code: {:?}", code);

    // part one
    let (acc_value, sequence, _) = process_code(&code);
    //println!("sequence: {:?}", sequence);
    println!("==result==");
    println!("{}", acc_value);

    // part two
    let mut acc_when_terminates = 0;
    for n in sequence.iter().rev() {
        // check for every command we run
        // to mangle it (if jmp -> nop, if nop -> jmp)
        let mut mangled_code = code.clone();
        if mangled_code[&n].cmd == "jmp" {
            mangled_code.get_mut(n).unwrap().cmd = "nop".to_string();
        } else if mangled_code[&n].cmd == "nop" {
            mangled_code.get_mut(n).unwrap().cmd = "jmp".to_string();
        } else {
            continue;
        }

        println!("mangling line {}", n);
        let (acc, _, terminated) = process_code(&mangled_code);
        if terminated {
            // if code terminates then we found it
            acc_when_terminates = acc;
            break;
        }
    }
    println!("==result part two==");
    println!("{}", acc_when_terminates);
}
