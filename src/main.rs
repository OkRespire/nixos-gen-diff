use std::io;

use crate::{
    model::PackageChanges,
    nix::generation::{diff_generations, list_generations},
};

mod model;
mod nix;

fn main() {
    let generations = list_generations();
    for generation in &generations {
        println!(
            "gen no: {} build date: {}",
            generation.number, generation.build_date
        );
    }
    println!("Choose a generation number:");
    let old_num = read_generation_number();

    println!("Choose another generation number:");
    let new_num = read_generation_number();

    let (old_num, new_num) = if old_num > new_num {
        (new_num, old_num)
    } else {
        (old_num, new_num)
    };
    let old = generations
        .iter()
        .find(|g| g.number == old_num)
        .expect("Generation not found");

    let new = generations
        .iter()
        .find(|g| g.number == new_num)
        .expect("Generation not found");

    let pkgs = diff_generations(old, new);
    eprintln!("{:#?}", &pkgs);

    let changes = PackageChanges::from_packages(pkgs);
    println!("{}", changes)
}

fn read_generation_number() -> u32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
