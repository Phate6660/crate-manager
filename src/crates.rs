use std::io::Read;

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub external_deps: Vec<String>,
}

/// Reads the list of crates from `$HOME/exported_crates.txt` and returns a vector of `Crate`s.
pub fn list_exported(crates_file: &str) -> Vec<Crate> {
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
pub fn list_cargos(crates_file: &str, manager_rules_file: &str) -> Vec<Crate> {
    let crates = std::fs::File::open(crates_file);
    let manager_rules = std::fs::File::open(manager_rules_file);
    let crates_list = if let Ok(mut f) = crates {
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents
    } else {
        String::new()
    };
    if crates_list.is_empty() {
        println!("Failed to read file because: {}", crates_file);
        // Exit with an error code.
        // TODO: Return an error instead.
        std::process::exit(1);
    }
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
        // Use an empty vector if the rules file is missing.
        vec![(String::new(), vec![String::new()])]
    };
    let mut crates_vec: Vec<Crate> = Vec::new();
    for (idx, line) in crates_list.lines().enumerate() {
        // Skip the section header.
        if idx == 0 {
            continue;
        }
        // Only add crates that were installed from a registry.
        if !line.contains("registry") {
            continue;
        }
        let parts = line.split('=').collect::<Vec<&str>>();
        let name_and_version_string = parts[0].trim().replace('"', "");
        let name_and_version = name_and_version_string.split(' ').collect::<Vec<&str>>();
        // If the crate matches a name in the rules file, use the deps specified in the rules file.
        // Otherwise use as an empty vec (no external deps).
        if rules.iter().find(|rule| rule.0 == name_and_version[0]).is_some() {
            let name = name_and_version[0].to_string();
            let version = name_and_version[1].to_string();
            let external_deps = rules.iter().find(|r| r.0 == name).unwrap().1.clone();
            let single_crate = Crate {
                name,
                version,
                external_deps,
            };
            crates_vec.push(single_crate);
        } else {
            let single_crate = Crate {
                name: name_and_version[0].to_string(),
                version: name_and_version[1].to_string(),
                external_deps: vec![],
            };
            crates_vec.push(single_crate);
        }
    }
    crates_vec
}

/// A quick helper function for checking the deps of a crate.
/// Returns a bool so that it can be used in a `if` statement.
fn check_deps(single_crate: &Crate) -> bool {
    if !single_crate.external_deps.is_empty() {
        let name = &single_crate.name;
        let version = &single_crate.version;
        println!("{name} [{version}], which also depends on:");
        for external_dep in single_crate.external_deps.clone() {
            println!("- {external_dep}");
        }
        return true;
    }
    false
}

pub fn check(crates: &[Crate]) {
    for single_crate in crates.iter().cloned() {
        if check_deps(&single_crate) {
            continue;
        }
        let name = &single_crate.name;
        let version = &single_crate.version;
        println!("{name} [{version}]");
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

pub fn install(crates_list: Vec<Crate>, install_specific_versions: bool, excluded_packages: Vec<String>) {
    check(&crates_list);
    for single_crate in crates_list {
        if excluded_packages.contains(&single_crate.name) {
            continue;
        }
        if install_specific_versions {
            // $ cargo install <crate> --vers <version>
            run(&[
                "install",
                &single_crate.name,
                "--vers",
                &single_crate.version,
            ]);
        } else {
            // $ cargo install <crate>
            run(&["install", &single_crate.name]);
        }
    }
}
