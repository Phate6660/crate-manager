mod crates;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let user = std::env::var("USER").unwrap();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/".to_string() + &user);
    // `$HOME/.cargo/.crates.toml` must exist!
    // TODO: Check if `$HOME/.cargo/.crates.toml` exists, and if not create it as an empty file.
    let cargos_crates_file = format!("{}/{}", home, "/.cargo/.crates.toml");
    let manager_rules_file = format!("{}/{}", home, "/.cm_rules");
    let cargos_crates_vec = crates::list_cargos(&cargos_crates_file, &manager_rules_file);
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
                if bin_crate.external_deps.is_empty() {
                    line = format!("{}={}", bin_crate.name, bin_crate.version);
                } else {
                    line = format!(
                        "{}={}={}",
                        bin_crate.name,
                        bin_crate.version,
                        bin_crate.external_deps.join(",")
                    );
                }
                writeln!(stored_crates_file, "{}", line).unwrap();
            }
        },
        "install" => {
            let get_specific_versions: bool = if args.get(2).is_some() {
                args.get(2).unwrap().parse::<bool>().unwrap()
            } else {
                false
            };
            if std::path::Path::new(&stored_crates).exists() {
                let crates_vec = crates::list_exported(&stored_crates);
                crates::install(crates_vec, get_specific_versions);
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
