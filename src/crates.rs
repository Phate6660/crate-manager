#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub external_deps: Vec<String>
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
