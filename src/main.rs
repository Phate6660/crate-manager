use std::io::Read;

fn main() {
    let user = std::env::var("USER").unwrap();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/".to_string() + &user);
    let crates_file = home + "/.cargo/.crates.toml";
    let crates = std::fs::File::open(crates_file);
    let crates_list = match crates {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            contents
        }
        Err(_) => {
            let mut crates = String::new();
            std::io::stdin().read_to_string(&mut crates).unwrap();
            crates
        }
    };
    let mut crates_vec: Vec<(&str, &str)> = Vec::new();
    for (idx, line) in crates_list.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let parts = line.split("=").collect::<Vec<&str>>();
        let name_and_version_string = parts[0].trim();
        let name = parts[1].trim();
        crates_vec.push((name_and_version_string, name));
    }
    let mut final_crates_vec: Vec<(&str, &str)> = Vec::new();
    for single_crate in crates_vec {
        let name_and_version_string = single_crate.0;
        if name_and_version_string.contains("registry") {
            final_crates_vec.push(single_crate);
        }
    }
    for bin_crate in final_crates_vec {
        let mut name = bin_crate.0;
        let name_without_quotes = &name.replace("\"", "");
        name = name_without_quotes.split(' ').collect::<Vec<&str>>()[0];
        println!("{}", name);
    }
}
