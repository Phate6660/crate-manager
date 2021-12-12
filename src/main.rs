use std::fs::OpenOptions;
use std::io::{Read, Write};

fn list_crates(crates_file: &str) -> Vec<String> {
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
    let mut crates_vec: Vec<&str> = Vec::new();
    for (idx, line) in crates_list.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let parts = line.split("=").collect::<Vec<&str>>();
        let name_and_version_string = parts[0].trim();
        crates_vec.push(name_and_version_string);
    }
    let mut final_crates_vec: Vec<String> = Vec::new();
    for single_crate in crates_vec {
        if single_crate.contains("registry") {
            final_crates_vec.push(single_crate.to_string());
        }
    }
    final_crates_vec
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let user = std::env::var("USER").unwrap();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/".to_string() + &user);
    let crates_file = format!("{}/{}", home, "/.cargo/.crates.toml");
    let crates_vec = list_crates(&crates_file);
    let na = String::from("N/A");
    let op = args.get(1).or(Some(&na)).unwrap();
    if op == "export" {
        let stored_crates = format!("{}/{}", home, "exported_crates.txt");
        let mut options = OpenOptions::new();
        let mut stored_crates_file = options
            .create(true)
            .append(true)
            .open(stored_crates)
            .unwrap();
        for bin_crate in crates_vec {
            let mut name = bin_crate;
            let name_without_quotes = &name.replace("\"", "");
            name = name_without_quotes.split(' ').collect::<Vec<&str>>()[0].to_string();
            writeln!(stored_crates_file, "{}", name).unwrap();
        }
    } else if op == "list" {
        for bin_crate in crates_vec {
            let mut name = bin_crate;
            let name_without_quotes = &name.replace("\"", "");
            name = name_without_quotes.split(' ').collect::<Vec<&str>>()[0].to_string();
            println!("{}", name);
        }
    } else {
        println!("list (to list installed crates) or export (to export said crates to ~/exported_crates.txt)");
    }
}
