use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        // birth year
        match &self.byr {
            Some(year) => {
                let year: u32 = year.to_string().parse().unwrap_or_default();
                if year < 1920 || year > 2002 {
                    return false;
                }
            }
            None => return false,
        }

        // issue year
        match &self.iyr {
            Some(year) => {
                let year: u32 = year.to_string().parse().unwrap_or_default();
                if year < 2010 || year > 2020 {
                    return false;
                }
            }
            None => return false,
        }

        // expiration year
        match &self.eyr {
            Some(year) => {
                let year: u32 = year.to_string().parse().unwrap_or_default();
                if year < 2020 || year > 2030 {
                    return false;
                }
            }
            None => return false,
        }

        // height
        match &self.hgt {
            Some(height) => {
                if !height.ends_with("cm") && !height.ends_with("in") {
                    return false;
                }

                if height.ends_with("cm") {
                    if height.len() != 5 {
                        return false;
                    }
                    let number: u16 = height
                        .chars()
                        .take(3)
                        .collect::<String>()
                        .parse()
                        .unwrap_or_default();
                    if number < 150 || number > 193 {
                        return false;
                    }
                }

                if height.ends_with("in") {
                    if height.len() != 4 {
                        return false;
                    }
                    let number: u16 = height
                        .chars()
                        .take(2)
                        .collect::<String>()
                        .parse()
                        .unwrap_or_default();
                    if number < 59 || number > 76 {
                        return false;
                    }
                }
            }
            None => return false,
        }

        // hair color
        match &self.hcl {
            Some(color) => {
                if color.len() != 7 {
                    return false;
                }
                let color: Vec<char> = color.chars().collect();
                if let Some((hash_sign, color_chars)) = color.split_first() {
                    if *hash_sign != '#' {
                        return false;
                    }

                    match u32::from_str_radix(&color_chars.iter().collect::<String>(), 16) {
                        Ok(number) => number,
                        Err(_) => return false,
                    };
                }
            }
            None => return false,
        }

        // eye color
        match &self.ecl {
            Some(color) => {
                let valid_color_codes = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                if !valid_color_codes.contains(&color.as_str()) {
                    return false;
                }
            }
            None => return false,
        }

        // passport id
        match &self.pid {
            Some(pid) => {
                if pid.len() != 9 {
                    return false;
                }
                match pid.parse::<u64>() {
                    Ok(number) => {
                        if number >= 1_000_000_000 {
                            return false;
                        } else {
                            number
                        }
                    }
                    Err(_) => return false,
                };
            }
            None => return false,
        }

        true
    }
}

fn read_passports(filename: String) -> Vec<Passport> {
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

    // pre-process
    // build `lines` which is vector of line passports
    let lines: Vec<&str> = content.split("\n\n").collect();
    let lines: Vec<String> = lines.iter().map(|s| s.trim().replace("\n", " ")).collect();

    // process
    // build `passports` which is vector of struct Passport
    let mut passports: Vec<Passport> = vec![];
    for line in lines {
        let fields: Vec<&str> = line.split(' ').collect();
        let mut new_passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for f in fields {
            let kv_pair: Vec<&str> = f.split(':').collect();
            let value = Some(kv_pair[1].to_string());
            match kv_pair[0] {
                "byr" => new_passport.byr = value,
                "iyr" => new_passport.iyr = value,
                "eyr" => new_passport.eyr = value,
                "hgt" => new_passport.hgt = value,
                "hcl" => new_passport.hcl = value,
                "ecl" => new_passport.ecl = value,
                "pid" => new_passport.pid = value,
                "cid" => new_passport.cid = value,
                _ => {}
            }
        }
        passports.push(new_passport);
    }

    passports
}

fn main() {
    let filename = String::from("./src/input");
    let passports = read_passports(filename);

    // part one
    let mut valid_count = 0;
    for p in &passports {
        if p.byr.is_some()
            && p.iyr.is_some()
            && p.eyr.is_some()
            && p.hgt.is_some()
            && p.hcl.is_some()
            && p.ecl.is_some()
            && p.pid.is_some()
        {
            valid_count += 1;
        }
    }
    println!("==result==");
    println!("{}", valid_count);

    // part two
    let mut valid_count = 0;
    for p in &passports {
        if p.is_valid() {
            valid_count += 1;
        }
    }
    println!("==result part two==");
    println!("{}", valid_count);
}
