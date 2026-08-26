use anyhow::{Context, Result};
use std::process::Command;

use crate::{
    model::{Generation, PackageChange},
    nix::diff::parse_packages,
};

pub fn list_generations() -> Result<Vec<Generation>> {
    let output = Command::new("nixos-rebuild")
        .arg("list-generations")
        .output()
        .context("nixos-rebuild not found - is nixos-rebuild installed?")?;

    let stdout = String::from_utf8(output.stdout).context("Cannot parse output into a string")?;

    stdout.lines().skip(1).map(parse_generation).collect()
}
fn parse_generation(input: &str) -> Result<Generation> {
    let fields: Vec<&str> = input.split_whitespace().collect();
    let is_curr = if fields[7] == "True" { true } else { false };
    let link = std::fs::read_link(format!("/nix/var/nix/profiles/system-{}-link", fields[0]))
        .context("Link not found")?;
    let number = fields[0].parse().context("Not a number.")?;
    Ok(Generation {
        number,
        link,
        build_date: format!("{} {}", fields[1], fields[2]),
        ver: fields[3].to_string(),
        kernel: fields[4].to_string(),
        is_current: is_curr,
    })
}

pub fn diff_generations(old: &Generation, new: &Generation) -> Result<Vec<PackageChange>> {
    let output = Command::new("nix")
        .arg("store")
        .arg("diff-closures")
        .arg(&old.link)
        .arg(&new.link)
        .output()
        .context("nix not found - is nix installed?")?;

    let stdout =
        String::from_utf8(output.stdout).context("Unable to parse output into a string")?;

    stdout.lines().map(parse_packages).collect()
}
