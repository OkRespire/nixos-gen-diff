use std::{fmt::Display, io, path::PathBuf, process::Command};

use strip_ansi_escapes::strip_str;

mod model;
mod nix;

enum PackageChangeType {
    Kernel,
    Updated,
    Removed,
    Added,
    Rebuilt,
    Other,
}

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
    old_ver: Vec<String>,
    new_ver: Vec<String>,
    size_delta: Option<Size>,
}

impl Display for PackageChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pkg_name)?;

        if !self.old_ver.is_empty() {
            write!(f, ": ")?;
            for (i, ver) in self.old_ver.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{ver}")?;
            }
        }

        if !self.new_ver.is_empty() {
            if self.old_ver.is_empty() {
                write!(f, ": ")?;
            } else {
                write!(f, " -> ")?;
            }
            for (i, ver) in self.new_ver.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{ver}")?;
            }
        }

        if self.old_ver.is_empty() {
            write!(f, ": ")?;
        }
        if let Some(sd) = &self.size_delta {
            if sd.delta != 0.0 {
                if self.old_ver.is_empty() && self.new_ver.is_empty() {
                    write!(f, "{}", sd)?;
                } else {
                    write!(f, ", {}", sd)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Size {
    pub delta: f64,
    pub unit: String,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.delta, self.unit)
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            delta: 0.0,
            unit: "".to_string(),
        }
    }
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
        (new_store_path, old_store_path)
    } else {
        (old_store_path, new_store_path)
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
    eprintln!("{:#?}", &pkgs);

    let mut kernel = Vec::new();
    let mut updated = Vec::new();
    let mut removed = Vec::new();
    let mut added = Vec::new();
    let mut rebuild = Vec::new();
    let mut other = Vec::new();

    for p in pkgs {
        match classify_package(&p) {
            PackageChangeType::Kernel => kernel.push(p),
            PackageChangeType::Updated => updated.push(p),
            PackageChangeType::Removed => removed.push(p),
            PackageChangeType::Added => added.push(p),
            PackageChangeType::Rebuilt => rebuild.push(p),
            PackageChangeType::Other => other.push(p),
        }
    }

    println!("--- Package Update Summary ---");
    if !kernel.is_empty() {
        println!("[Kernel Updates]");
        print_vec(kernel);
    }
    if !updated.is_empty() {
        println!("[Updated Packages]");
        print_vec(updated);
    }
    if !removed.is_empty() {
        println!("[Removed Packages]");
        print_vec(removed);
    }
    if !added.is_empty() {
        println!("[Added Packages]");
        print_vec(added);
    }
    if !rebuild.is_empty() {
        println!("[Rebuilt Packages]");
        print_vec(rebuild);
    }
    if !other.is_empty() {
        println!("[Other Changes]");
        print_vec(other);
    }
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
    let fields: Vec<&str> = input
        .split_whitespace()
        .map(|s| s.trim_end_matches(","))
        .collect();

    eprintln!("DEBUG fields: {:?}", fields);
    let name = fields[0].trim_end_matches(":").to_string();
    let Some(arrow_idx) = fields.iter().position(|f| *f == "→") else {
        let delta: f64 = fields[1].parse().unwrap();
        let size = Some(Size {
            delta,
            unit: fields[2].to_string(),
        });
        return PackageChange {
            pkg_name: name,
            old_ver: Vec::new(),
            new_ver: Vec::new(),
            size_delta: size,
        };
    };

    let mut new_end = fields.len();
    let mut size_delta = None;

    if fields.len() >= arrow_idx + 3 {
        let delta_idx = fields.len() - 2;
        if let Ok(delta) = fields[delta_idx].parse::<f64>() {
            size_delta = Some(Size {
                delta,
                unit: fields[delta_idx + 1].to_string(),
            });
        }
        new_end = delta_idx
    }

    let mut old_ver: Vec<String> = fields[1..arrow_idx]
        .iter()
        .map(|s| s.trim_end_matches(',').to_string())
        .collect();

    let mut new_ver: Vec<String> = fields[arrow_idx + 1..new_end]
        .iter()
        .map(|s| s.trim_end_matches(',').to_string())
        .collect();

    if old_ver.len() == 1 && (old_ver[0] == "∅" || old_ver[0] == "ε") {
        old_ver.clear();
    }

    if new_ver.len() == 1 && (new_ver[0] == "∅" || new_ver[0] == "ε") {
        new_ver.clear();
    }

    PackageChange {
        pkg_name: name,
        old_ver,
        new_ver,
        size_delta,
    }
}

fn print_vec(vec: Vec<PackageChange>) {
    for v in vec {
        println!("{}", v)
    }
    print!("\n")
}

fn is_kernel_package(name: &str) -> bool {
    matches!(
        name,
        "linux"
            | "linux-zen"
            | "linux-headers"
            | "linux-headers-static"
            | "initrd-linux-zen"
            | "linux-firmware"
    )
}

fn classify_package(p: &PackageChange) -> PackageChangeType {
    if is_kernel_package(&p.pkg_name) {
        PackageChangeType::Kernel
    } else if !p.old_ver.is_empty() && !p.new_ver.is_empty() {
        PackageChangeType::Updated
    } else if !p.old_ver.is_empty() {
        PackageChangeType::Removed
    } else if !p.new_ver.is_empty() {
        PackageChangeType::Added
    } else if p.size_delta.is_some() {
        PackageChangeType::Rebuilt
    } else {
        PackageChangeType::Other
    }
}
