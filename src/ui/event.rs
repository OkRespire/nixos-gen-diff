use std::{sync::mpsc, thread};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    model::PackageChanges,
    nix::generation::diff_generations,
    ui::{
        CATEGORIES, {DiffApp, Screen},
    },
};

pub fn handle_key(app: &mut DiffApp, key: KeyCode) {
    match &mut app.screen {
        Screen::SelectGenerations {
            generations,
            cursor,
            picked,
        } => match key {
            KeyCode::Down => *cursor = (*cursor + 1).min(generations.len() - 1),
            KeyCode::Up => *cursor = cursor.saturating_sub(1),
            KeyCode::Enter => {
                let gens = generations[*cursor].clone();
                if picked.contains(&gens) {
                    picked.retain(|n| *n != gens);
                } else if picked.len() < 2 {
                    picked.push(gens);
                }
            }
            KeyCode::Char(' ') => {
                if picked.len() == 2 {
                    let (old, new) = if picked[0].number > picked[1].number {
                        (picked[1].clone(), picked[0].clone())
                    } else {
                        (picked[0].clone(), picked[1].clone())
                    };
                    let (tx, rx) = mpsc::channel();
                    let (thread_old, thread_new) = (old.clone(), new.clone());
                    thread::spawn(move || {
                        let result = diff_generations(&thread_old, &thread_new);
                        let _ = tx.send(result);
                    });
                    app.screen = Screen::Diffing { old, new, rx }
                }
            }
            KeyCode::Esc => app.should_quit = true,
            _ => {}
        },
        Screen::Results {
            changes,
            active_tab,
            cursor,
        } => {
            let length = match active_tab {
                0 => changes.kernel.len(),
                1 => changes.updated.len(),
                2 => changes.removed.len(),
                3 => changes.added.len(),
                4 => changes.rebuilt.len(),
                5 => changes.other.len(),
                _ => unreachable!(),
            };
            match key {
                KeyCode::Tab => *active_tab = (*active_tab + 1).min(CATEGORIES.len() - 1),
                KeyCode::BackTab => *active_tab = active_tab.saturating_sub(1),
                KeyCode::Esc => app.should_quit = true,
                KeyCode::Char('0') => *active_tab = 0,
                KeyCode::Char('1') => *active_tab = 1,
                KeyCode::Char('2') => *active_tab = 2,
                KeyCode::Char('3') => *active_tab = 3,
                KeyCode::Char('4') => *active_tab = 4,
                KeyCode::Char('5') => *active_tab = 5,
                KeyCode::Down => {
                    *cursor = (*cursor + 1).min(length - 1);
                }
                KeyCode::Up => *cursor = cursor.saturating_sub(1),
                KeyCode::Char('j') => *cursor = (*cursor + 1).min(length - 1),
                KeyCode::Char('k') => *cursor = cursor.saturating_sub(1),
                _ => {}
            }
        }

        _ => {}
    }
}

pub fn handle_events(app: &mut DiffApp, evt: Event) -> Result<()> {
    if let Event::Key(key_event) = evt {
        if key_event.kind == KeyEventKind::Press {
            handle_key(app, key_event.code);
        }
    }
    match &mut app.screen {
        Screen::SelectGenerations { .. } => {}
        Screen::Diffing { .. } => {
            // let pkgs = diff_generations(old, new)?;
            // let changes = PackageChanges::from_packages(pkgs);
            // app.screen = Screen::Results {
            //     changes,
            //     active_tab: 0,
            //     cursor: 0,
            // }
        }
        Screen::Results { .. } => {}
    }
    Ok(())
}
