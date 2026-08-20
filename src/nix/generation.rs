use std::process::Command;

use crate::{
    model::{Generation, PackageChange},
    nix::diff::parse_packages,
};

pub fn list_generations() -> Vec<Generation> {
    let output = Command::new("nixos-rebuild")
        .arg("list-generations")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout.lines().skip(1).map(parse_generation).collect()
}
fn parse_generation(input: &str) -> Generation {
    let fields: Vec<&str> = input.split_whitespace().collect();
    let is_curr = if fields[7] == "True" { true } else { false };
    let link =
        std::fs::read_link(format!("/nix/var/nix/profiles/system-{}-link", fields[0])).unwrap();
    Generation {
        number: fields[0].parse().unwrap(),
        link,
        build_date: format!("{} {}", fields[1], fields[2]),
        ver: fields[3].to_string(),
        kernel: fields[4].to_string(),
        is_current: is_curr,
    }
}

pub fn diff_generations(old: &Generation, new: &Generation) -> Vec<PackageChange> {
    let output = Command::new("nix")
        .arg("store")
        .arg("diff-closures")
        .arg(&old.link)
        .arg(&new.link)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout.lines().map(parse_packages).collect()
}
