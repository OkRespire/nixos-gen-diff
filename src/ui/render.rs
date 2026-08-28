use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    widgets::{List, ListItem, ListState, Paragraph, Tabs},
};

use crate::{
    model::{Generation, PackageChange, PackageChanges},
    ui::{
        CATEGORIES,
        app::{DiffApp, Screen},
    },
};

pub fn draw_ui(f: &mut Frame<'_>, app: &DiffApp) {
    match &app.screen {
        Screen::SelectGenerations {
            generations,
            cursor,
            picked,
        } => render_gens_screen(f, generations, cursor, picked),
        Screen::Diffing { old, new, .. } => {
            render_diff_screen(f, old, new);
        }
        Screen::Results {
            changes,
            active_tab,
            cursor,
        } => {
            render_res_screen(f, changes, active_tab, cursor);
        }
    }
}

fn render_gens_screen(
    f: &mut Frame<'_>,
    gens: &Vec<Generation>,
    cursor: &usize,
    picked: &Vec<Generation>,
) {
    let items: Vec<ListItem> = gens
        .into_iter()
        .map(|s| -> ListItem<'_> {
            let line = format!("Number: {} \t Build Date: {}", s.number, s.build_date);

            if picked.contains(&s) {
                ListItem::new(line).style(Style::default().fg(Color::White).bg(Color::Green))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items).highlight_style(Style::default().reversed());

    let mut list_state = ListState::default().with_selected(Some(*cursor));

    f.render_stateful_widget(list, f.area(), &mut list_state);
}

fn render_diff_screen(f: &mut Frame<'_>, old: &Generation, new: &Generation) {
    let para = Paragraph::new(format!(
        "Diffing generations {} -> {}",
        old.number, new.number
    ))
    .centered();

    f.render_widget(para, f.area());
}

fn render_res_screen(
    f: &mut Frame<'_>,
    changes: &PackageChanges,
    active_tab: &usize,
    cursor: &usize,
) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = f.area().layout(&layout);

    let tabs = Tabs::new(CATEGORIES)
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(*active_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");

    let text = match active_tab {
        0 => format_package_list(&changes.kernel),
        1 => format_package_list(&changes.updated),
        2 => format_package_list(&changes.removed),
        3 => format_package_list(&changes.added),
        4 => format_package_list(&changes.rebuilt),
        5 => format_package_list(&changes.other),
        _ => unreachable!(),
    }
    .highlight_style(Style::default().reversed());

    let mut list_state = ListState::default().with_selected(Some(*cursor));
    f.render_widget(tabs, top);
    f.render_stateful_widget(text, main, &mut list_state);
}

fn format_package_list(list: &[PackageChange]) -> List<'_> {
    let thing: Vec<ListItem> = list
        .iter()
        .map(|p| -> ListItem<'_> { ListItem::new(p.to_string()) })
        .collect();

    List::new(thing)
}
