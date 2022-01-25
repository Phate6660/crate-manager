mod crates;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user = std::env::var("USER").unwrap();
    let or_home = format!("/home/{user}");
    let home = std::env::var("HOME").unwrap_or(or_home);
    let stored_crates = format!("{home}/exported_crates.txt");
    let na = String::from("N/A");
    let op = args.get(1).or(Some(&na)).unwrap();
    match op.as_str() {
        "export" => {
            // `$HOME/.cargo/.crates.toml` must exist!
            // From my testing it's automatically created once you install a crate through cargo.
            // And you shouldn't be running export anyway without having installed something.
            let cargos_crates_file = format!("{home}/.cargo/.crates.toml");
            let manager_rules_file = format!("{home}/.cm_rules");
            let cargos_crates_vec = crates::list_cargos(&cargos_crates_file, &manager_rules_file);
            let mut options = OpenOptions::new();
            let mut stored_crates_file = options
                .create(true)
                .append(true)
                .open(stored_crates)
                .unwrap();
            for bin_crate in cargos_crates_vec {
                let name = bin_crate.name;
                let version = bin_crate.version;
                let line = if bin_crate.external_deps.is_empty() {
                    format!("{name}={version}")
                } else {
                    let external_deps = bin_crate.external_deps.join(",");
                    format!("{name}={version}={external_deps}")
                };
                writeln!(stored_crates_file, "{}", line).unwrap();
            }
        },
        "install" => {
            // TODO: Parse the command line arguments much better.
            // This currently relies on specific argument placement and it's not very flexible.
            // `--exclude pkg,pkg2` is required to be passed first, followed by the bool
            // for specifying whether to install specific crate versions.
            let mut excluded_packages: Vec<String> = Vec::new();
            let mut install_specific_versions = false;
            if args.get(2).or(Some(&na)).unwrap() == "--exclude" {
                let excluded_packages_string = args.get(3).or(Some(&na)).unwrap();
                let mut excluded_packages_vec = Vec::new();
                for excluded_package in excluded_packages_string.split(",") {
                    excluded_packages_vec.push(excluded_package.to_string());
                }
                excluded_packages = excluded_packages_vec;
            } else {
                if args.get(4).is_some() {
                    install_specific_versions = args.get(2).unwrap().parse::<bool>().unwrap();
                } else {
                    install_specific_versions = false;
                };
            }
            if std::path::Path::new(&stored_crates).exists() {
                let crates_vec = crates::list_exported(&stored_crates);
                crates::install(crates_vec, install_specific_versions, excluded_packages);
            } else {
                println!("Crates have not been exported yet. Please use the 'export' command.");
            }
        },
        "list" => {
            let crates_vec = crates::list_exported(&stored_crates);
            crates::check(&crates_vec);
        },
        _ => println!("export (to export installed crates to ~/exported_crates.txt)\n\
                       install (to install exported crates)\n\
                       install true (to install exported crates with specific versions)\n\
                       list (to list installed crates)"),
    }
}
