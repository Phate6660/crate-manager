use std::io::Read;

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub external_deps: Vec<String>
}

pub fn list_crates(crates_file: &str) -> Vec<Crate> {
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

pub fn list_cargos_crates(crates_file: &str) -> Vec<Crate> {
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

fn check_crate_deps(single_crate: &Crate) {
    if !single_crate.external_deps.is_empty() {
        println!("{} [{}], which also depends on:", single_crate.name, single_crate.version);
        for external_dep in single_crate.external_deps.clone() {
            println!("- {}", external_dep);
        }
    }
}

pub fn check_crates(crates: &Vec<Crate>, check_deps: bool) {
    for single_crate in crates.clone() {
        if check_deps {
            check_crate_deps(&single_crate);
        } else {
            check_crate_deps(&single_crate);
            println!("{} [{}]", single_crate.name, single_crate.version);
        }
    }
}

pub fn install_crates(crates_list: Vec<Crate>) {
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
