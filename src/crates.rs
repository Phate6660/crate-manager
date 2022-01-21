use std::io::Read;

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub external_deps: Vec<String>,
}

/// Reads the list of crates from `$HOME/exported_crates.txt` and returns a vector of `Crate`s.
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
            let external_deps = parts[2]
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            crates.push(Crate {
                name: name.to_string(),
                version: version.to_string(),
                external_deps,
            });
        }
    }
    crates
}

/// Reads Cargo's own list of crates and as well as a file of rules,
/// and uses them to return a vector of `Crate`s.
pub fn list_cargos_crates(crates_file: &str, manager_rules_file: &str) -> Vec<Crate> {
    let crates = std::fs::File::open(crates_file);
    let manager_rules = std::fs::File::open(manager_rules_file);
    let crates_list = if let Ok(mut f) = crates {
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents
    } else {
        let mut crates = String::new();
        std::io::stdin().read_to_string(&mut crates).unwrap();
        crates
    };
    let rules: Vec<(String, Vec<String>)> = if let Ok(mut f) = manager_rules {
        let mut tmp_rules = Vec::new();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            let parts = line.split('=').collect::<Vec<&str>>();
            let name = parts[0];
            let external_deps = parts[1]
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            let rule = (name.to_string(), external_deps);
            tmp_rules.append(&mut vec![rule]);
        }
        tmp_rules
    } else {
        let mut tmp_rules = Vec::new();
        let mut rules = String::new();
        std::io::stdin().read_to_string(&mut rules).unwrap();
        let parts = rules.split('=').collect::<Vec<&str>>();
        let name = parts[0];
        let external_deps = parts[1]
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let rule = (name.to_string(), external_deps);
        tmp_rules.append(&mut vec![rule]);
        tmp_rules
    };
    let mut crates_vec: Vec<Crate> = Vec::new();
    for (idx, line) in crates_list.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        if !line.contains("registry") {
            continue;
        }
        let parts = line.split('=').collect::<Vec<&str>>();
        let name_and_version_string = parts[0].trim().replace('"', "");
        let name_and_version = name_and_version_string.split(' ').collect::<Vec<&str>>();
        for rule in &rules {
            if rule.0 == name_and_version[0] {
                let name = name_and_version[0].to_string();
                let version = name_and_version[1].to_string();
                let external_deps = rule.1.clone();
                crates_vec.push(Crate {
                    name,
                    version,
                    external_deps,
                });
            } else {
                crates_vec.push(Crate {
                    name: name_and_version[0].to_string(),
                    version: name_and_version[1].to_string(),
                    external_deps: vec![],
                });
            }
        }
    }
    crates_vec
}

/// A quick helper function for checking the deps of a crate.
fn check_crate_deps(single_crate: &Crate) {
    if !single_crate.external_deps.is_empty() {
        println!(
            "{} [{}], which also depends on:",
            single_crate.name, single_crate.version
        );
        for external_dep in single_crate.external_deps.clone() {
            println!("- {}", external_dep);
        }
    }
}

pub fn check_crates(crates: &[Crate], check_deps: bool) {
    for single_crate in crates.to_owned() {
        if check_deps {
            check_crate_deps(&single_crate);
        } else {
            check_crate_deps(&single_crate);
            println!("{} [{}]", single_crate.name, single_crate.version);
        }
    }
}

/// A quick helper function for running a command.
fn run(args: &[&str]) {
    let child = std::process::Command::new("cargo")
        .args(args)
        .spawn()
        .expect("failed to execute process");
    let output = child.wait_with_output().unwrap().stdout;
    let usable_output = std::str::from_utf8(&output).unwrap();
    println!("{}", usable_output);
}

pub fn install_crates(crates_list: Vec<Crate>, get_specific_versions: bool) {
    check_crates(&crates_list, true);
    for single_crate in crates_list {
        if get_specific_versions {
            // $ cargo install <crate> --vers <version>
            run(&["install", &single_crate.name, "--vers", &single_crate.version]);
        } else {
            // $ cargo install <crate>
            run(&["install", &single_crate.name]);
        }
            
    }
}
