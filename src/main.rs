use std::{io, path::PathBuf, process::Command};

use strip_ansi_escapes::strip_str;

#[derive(Debug)]
struct Generation {
    number: u32,
    link: PathBuf,
    build_date: String,
    ver: String,
    kernel: String,
    is_current: bool,
}

#[derive(Debug)]
struct PackageChange {
    pkg_name: String,
    old_ver: Option<String>,
    new_ver: Option<String>,
    size_delta: Option<Size>,
}

#[derive(Debug)]
struct Size {
    delta: f64,
    unit: String,
}

fn main() {
    let output = Command::new("nixos-rebuild")
        .arg("list-generations")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let gens = stdout
        .lines()
        .skip(1)
        .map(|line| parse_as_gen(line))
        .collect::<Vec<Generation>>();
    for x in &gens {
        println!("gen no: {} build date: {}", x.number, x.build_date)
    }

    println!("Choose the old generation number from the above");
    let mut buf_old = String::new();
    io::stdin().read_line(&mut buf_old).unwrap();
    let old_num: u32 = buf_old.trim().parse().unwrap();

    println!("Choose the new generation number from the above");
    let mut buf_new = String::new();
    io::stdin().read_line(&mut buf_new).unwrap();
    let new_num: u32 = buf_new.trim().parse().unwrap();

    println!("{} {}", old_num, new_num);

    let old_store_path = if let Some(x) = gens.iter().find(|x| x.number == old_num) {
        &x.link
    } else {
        panic!("Not found")
    };

    let new_store_path = if let Some(x) = gens.iter().find(|x| x.number == new_num) {
        &x.link
    } else {
        panic!("Not found")
    };
    let (new_store_path, old_store_path) = if old_num < new_num {
        (old_store_path, new_store_path)
    } else {
        (new_store_path, old_store_path)
    };

    let output = Command::new("nix")
        .arg("store")
        .arg("diff-closures")
        .arg(old_store_path)
        .arg(new_store_path)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let pkgs = stdout
        .lines()
        .map(|line| parse_as_packages(line))
        .collect::<Vec<PackageChange>>();
    println!("{:#?}", &pkgs);
}

fn parse_as_gen(input: &str) -> Generation {
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

fn parse_as_packages(input: &str) -> PackageChange {
    let input = strip_str(input);
    let fields: Vec<&str> = input.split_whitespace().collect();
    let name = fields[0].replace(":", "");
    if fields.len() == 3 {
        let delta: f64 = fields[1].parse().unwrap();
        let size = Some(Size {
            delta,
            unit: fields[2].to_string(),
        });
        return PackageChange {
            pkg_name: name,
            old_ver: None,
            new_ver: None,
            size_delta: size,
        };
    }
    let old_ver = if fields[1] == "∅" || fields[1] == "ε" {
        None
    } else {
        Some(fields[1].to_string())
    };
    let new_ver = if fields[3] == "∅," || fields[3] == "ε," {
        None
    } else {
        Some(fields[3].to_string())
    };

    let size_delta: Option<Size> = if fields.len() == 6 {
        eprintln!("DEBUG fields: {:?}", fields);
        let delta: f64 = fields[4].parse().unwrap();
        Some(Size {
            delta: delta,
            unit: fields[5].to_string(),
        })
    } else {
        None
    };

    PackageChange {
        pkg_name: name,
        old_ver: old_ver,
        new_ver: new_ver,
        size_delta: size_delta,
    }
}
