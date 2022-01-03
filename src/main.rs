use std::fs::OpenOptions;
use std::io::{Read, Write};

fn list_crates(crates_file: &str) -> Vec<(String, String)> {
    let mut crates = Vec::new();
    let file = std::fs::File::open(crates_file);
    let crates_file_contents = match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            contents
        }
        Err(e) => {
            println!("Failed to read file because: {}", e);
            String::new()
        }
    };
    for line in crates_file_contents.lines() {
        let mut parts = line.split('=');
        let name = parts.next().unwrap();
        let version = parts.next().unwrap();
        crates.push((name.to_string(), version.to_string()));
    }
    crates
}

fn list_cargos_crates(crates_file: &str) -> Vec<String> {
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
        let parts = line.split('=').collect::<Vec<&str>>();
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

fn install_crates(crates_list: Vec<(String, String)>) {
    for single_crate in crates_list {
        let command = format!("cargo install {} --vers {}", single_crate.0, single_crate.1);
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let user = std::env::var("USER").unwrap();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/".to_string() + &user);
    let cargos_crates_file = format!("{}/{}", home, "/.cargo/.crates.toml");
    let cargos_crates_vec = list_cargos_crates(&cargos_crates_file);
    let stored_crates = format!("{}/{}", home, "exported_crates.txt");
    let na = String::from("N/A");
    let op = args.get(1).or(Some(&na)).unwrap();
    match op.as_str() {
        "export" => {
            let mut options = OpenOptions::new();
            let mut stored_crates_file = options
                .create(true)
                .append(true)
                .open(stored_crates)
                .unwrap();
            for bin_crate in cargos_crates_vec {
                let name_without_quotes = bin_crate.replace("\"", "");
                let name_and_version = [
                    name_without_quotes.split(' ').collect::<Vec<&str>>()[0],
                    "=",
                    name_without_quotes.split(' ').collect::<Vec<&str>>()[1]
                ].concat();
                writeln!(stored_crates_file, "{}", name_and_version).unwrap();
            }
        },
        "install" => {
            if std::path::Path::new(&stored_crates).exists() {
                let crates_vec = list_crates(&stored_crates);
                // This is untested, but should work. Will remove this notice when tested.
                install_crates(crates_vec);
            } else {
                println!("Crates have not been exported yet. Please use the 'export' command.");
            }
        },
        "list" => {
            for bin_crate in cargos_crates_vec {
                let mut name = bin_crate;
                let name_without_quotes = &name.replace("\"", "");
                name = name_without_quotes.split(' ').collect::<Vec<&str>>()[0].to_string();
                println!("{}", name);
            }
        },
        _ => println!("export (to export installed crates to ~/exported_crates.txt)\ninstall (to install exported crates)\nlist (to list installed crates)"),
    }
}
