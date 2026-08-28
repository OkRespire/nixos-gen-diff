use std::time::Duration;

use anyhow::Result;
use crossterm::event;

use crate::{
    nix::generation::list_generations,
    ui::{DiffApp, Screen, app::advance, draw_ui, handle_events},
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
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|f| draw_ui(f, &app))?;

            advance(&mut app)?;
            if event::poll(Duration::from_millis(50))? {
                let event = event::read()?;
                handle_events(&mut app, event)?;
            }
            if app.should_quit {
                break anyhow::Ok(());
            }
        }
    })?;
    Ok(())
}

