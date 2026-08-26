use crossterm::event::KeyCode;

use crate::ui::app::{DiffApp, Screen};

const CATEGORIES: [&str; 6] = ["Kernel", "Updated", "Removed", "Added", "Rebuilt", "Other"];

pub fn handle_keys(app: &mut DiffApp, key: KeyCode) {
    match &mut app.screen {
        Screen::SelectGenerations {
            generations,
            cursor,
            picked,
        } => match key {
            KeyCode::Down => *cursor = (*cursor + 1).min(generations.len() - 1),
            KeyCode::Up => *cursor = cursor.saturating_sub(1),
            KeyCode::Enter => {
                let num = generations[*cursor].number;
                if picked.contains(&num) {
                    picked.retain(|n| *n != num);
                } else if picked.len() < 2 {
                    picked.push(num);
                }
            }
            KeyCode::Esc => app.should_quit = true,
            _ => {}
        },
        _ => todo!(),
    }
}
