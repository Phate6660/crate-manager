mod crates;
use crates::{check_crates, Crate};
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn list_crates(crates_file: &str) -> Vec<Crate> {
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
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            let name = parts[0];
            let version = parts[1];
            crates.push(Crate {
                name: name.to_string(),
                version: version.to_string(),
                external_deps: vec![],
            });
        } else if parts.len() == 3 {
            let name = parts[0];
            let version = parts[1];
            let external_deps = parts[2].split(',').collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
            crates.push(Crate {
                name: name.to_string(),
                version: version.to_string(),
                external_deps: external_deps,
            });
        }
    }
    crates
}

fn list_cargos_crates(crates_file: &str) -> Vec<Crate> {
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
    let mut crates_vec: Vec<Crate> = Vec::new();
    for (idx, line) in crates_list.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let parts = line.split('=').collect::<Vec<&str>>();
        let name_and_version_string = parts[0].trim().replace('"', "");
        let name_and_version = name_and_version_string.split(' ').collect::<Vec<&str>>();
        match name_and_version[0] {
            "pijul" => {
                crates_vec.push(Crate {
                    name: name_and_version[0].to_string(),
                    version: name_and_version[1].to_string(),
                    external_deps: vec![
                        "openssl".to_string(),
                        "libsodium".to_string(),
                        "libzstd".to_string(),
                        "xxhash".to_string(),
                        "pkg-config".to_string(),
                    ],
                });
            },
            _ => {
                crates_vec.push(Crate {
                    name: name_and_version[0].to_string(),
                    version: name_and_version[1].to_string(),
                    external_deps: vec![],
                });
            },
        }
    }
    crates_vec
}

fn install_crates(crates_list: Vec<Crate>) {
    check_crates(&crates_list, true);
    for single_crate in crates_list {
        let child = std::process::Command::new("cargo")
            .args(&["install", single_crate.name.to_string().as_str()])
            .spawn()
            .expect("failed to execute process");
        let output = child.wait_with_output().unwrap().stdout;
        let usable_output = std::str::from_utf8(&output).unwrap();
        println!("{}", usable_output);
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
                let line;
                if !bin_crate.external_deps.is_empty() {
                    line = format!("{}={}={}", bin_crate.name, bin_crate.version, bin_crate.external_deps.join(","));
                } else {
                    line = format!("{}={}", bin_crate.name, bin_crate.version);
                }
                writeln!(stored_crates_file, "{}", line).unwrap();
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
            let crates_vec = list_crates(&stored_crates);
            check_crates(&crates_vec, false);
        },
        _ => println!("export (to export installed crates to ~/exported_crates.txt)\ninstall (to install exported crates)\nlist (to list installed crates)"),
    }
}
