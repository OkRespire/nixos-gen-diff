use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{List, ListItem},
};

use crate::{
    model::Generation,
    ui::app::{DiffApp, Screen},
};

pub fn draw_ui(f: &mut Frame<'_>, app: &DiffApp) {
    match &app.screen {
        Screen::SelectGenerations {
            generations,
            cursor,
            picked,
        } => render_gens_screen(f, generations, cursor, picked),
        _ => todo!(),
    }
}

fn render_gens_screen(
    f: &mut Frame<'_>,
    gens: &Vec<Generation>,
    cursor: &usize,
    picked: &Vec<u32>,
) {
    let items: Vec<ListItem> = gens
        .into_iter()
        .enumerate()
        .map(|(i, s)| -> ListItem<'_> {
            let mut line = String::new();
            if i == *cursor {
                line = format!("> Number: {} \t Build Date: {}", s.number, s.build_date)
            } else {
                line = format!("Number: {} \t Build Date: {}", s.number, s.build_date)
            }

            if picked.contains(&s.number) {
                ListItem::new(line).style(Style::default().fg(Color::Black).bg(Color::Yellow))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items);

    f.render_widget(list, f.area());
}
