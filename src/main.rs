use std::io;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyEventKind};

use crate::{
    model::PackageChanges,
    nix::generation::{diff_generations, list_generations},
    ui::{
        app::{DiffApp, Screen},
        render, terminal,
    },
};

mod model;
mod nix;
mod ui;

fn main() -> Result<()> {
    let generations = list_generations()?;

    let mut app = DiffApp {
        screen: Screen::SelectGenerations {
            generations: generations,
            cursor: 0,
            picked: Vec::new(),
        },
        should_quit: false,
    };
    terminal::setup(|terminal| {
        loop {
            terminal.draw(|f| render::draw_ui(f, &app))?;
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    ui::event::handle_keys(&mut app, key_event.code)
                }
            }
            if app.should_quit {
                break Ok(());
            }
        }
    })?;
    // for generation in &generations {
    //     println!(
    //         "gen no: {} build date: {}",
    //         generation.number, generation.build_date
    //     );
    // }
    // println!("Choose a generation number:");
    // let old_num = read_generation_number()?;
    //
    // println!("Choose another generation number:");
    // let new_num = read_generation_number()?;
    //
    // let (old_num, new_num) = if old_num > new_num {
    //     (new_num, old_num)
    // } else {
    //     (old_num, new_num)
    // };
    //
    // let old = generations
    //     .iter()
    //     .find(|g| g.number == old_num)
    //     .expect("Generation not found");
    //
    // let new = generations
    //     .iter()
    //     .find(|g| g.number == new_num)
    //     .expect("Generation not found");
    //
    // let pkgs = diff_generations(old, new)?;
    // // eprintln!("{:#?}", &pkgs);
    //
    // let changes = PackageChanges::from_packages(pkgs);
    // println!("{}", changes);
    Ok(())
}

fn read_generation_number() -> Result<u32> {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .context("Cannot read the stdio into the buffer")?;
    buf.trim().parse().context("Cannot parse buffer into u32")
}
