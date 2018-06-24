use std::process::Command;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn remove_packages(packages: Vec<String>) {
    Command::new("pkg").arg("remove").args(packages).status().unwrap();
}

//TODO return for main, to avoid unwrapping
fn main() {
    let output = Command::new("pkg").arg("prime-list").output().unwrap();
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    let output = String::from_utf8_lossy(&output.stdout);
    let mut installed = HashSet::new();
    for line in output.lines() {
        installed.insert(line);
    }

    let mut content = String::new();
    let mut file = File::open("/home/kirillvr/pkg_prime_list").unwrap();
    file.read_to_string(&mut content).unwrap();

    let mut desired = HashSet::new();
    for line in content.lines() {
        desired.insert(line);
    }

    let to_be_installed = desired.difference(&installed);
    println!("{:?}", to_be_installed);
    let to_be_removed = installed.difference(&desired);
    println!("{:?}", to_be_removed);
    remove_packages(to_be_removed.map(|i| String::from(*i)).collect());
}
