use std::process::Command;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn remove_packages(packages: Vec<String>) {
    Command::new("pkg").arg("remove").args(packages).status().unwrap();
}

fn install_packages(packages: Vec<String>) {
    Command::new("pkg").arg("install").args(packages).status().unwrap();
}

fn autoremove_packages() {
    Command::new("pkg").arg("autoremove").status().unwrap();
}

//TODO return for main, to avoid unwrapping
fn main() {
    let output = Command::new("pkg").arg("prime-list").output().unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    let mut installed = HashSet::new();
    for line in output.lines() {
        installed.insert(line);
    }

    let mut content = String::new();
    //TODO hardcoded path
    let mut file = File::open("/home/kirillvr/pkg_prime_list").unwrap();
    file.read_to_string(&mut content).unwrap();

    let mut desired = HashSet::new();
    for line in content.lines() {
        desired.insert(line);
    }


    let to_be_removed: Vec<String> = installed.difference(&desired).map(|i| String::from(*i)).collect();
    if !to_be_removed.is_empty() {
        println!("To be removed: {:?}", to_be_removed);
        remove_packages(to_be_removed);
    }

    let to_be_installed: Vec<String> = desired.difference(&installed).map(|i| String::from(*i)).collect();
    if !to_be_installed.is_empty() {
        println!("To Be installed: {:?}", to_be_installed);
        install_packages(to_be_installed);
    }

    autoremove_packages();
}
