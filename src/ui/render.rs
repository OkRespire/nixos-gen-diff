use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
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
const SPINNER_FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub fn draw_ui(f: &mut Frame<'_>, app: &DiffApp) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
    let [content, footer] = f.area().layout(&layout);

    render_footer(f, footer, &app.screen);
    match &app.screen {
        Screen::SelectGenerations {
            generations,
            cursor,
            picked,
        } => render_gens_screen(f, content, generations, *cursor, picked),
        Screen::Diffing { old, new, .. } => {
            render_diff_screen(f, content, old, new);
        }
        Screen::Results {
            changes,
            active_tab,
            cursor,
        } => {
            render_res_screen(f, content, changes, *active_tab, *cursor);
        }
    }
}

fn render_gens_screen(
    f: &mut Frame<'_>,
    content: Rect,
    gens: &[Generation],
    cursor: usize,
    picked: &[Generation],
) {
    let items: Vec<ListItem> = gens
        .iter()
        .map(|s| -> ListItem<'_> {
            let line = format!("Number: {} \t Build Date: {}", s.number, s.build_date);

            if picked.contains(s) {
                ListItem::new(line).style(Style::default().fg(Color::White).bg(Color::Green))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items).highlight_style(Style::default().reversed());

    let mut list_state = ListState::default().with_selected(Some(cursor));

    f.render_stateful_widget(list, content, &mut list_state);
}

fn render_diff_screen(f: &mut Frame<'_>, content: Rect, old: &Generation, new: &Generation) {
    let para = Paragraph::new(format!(
        "Diffing generations {} -> {}",
        old.number, new.number
    ))
    .centered();

    f.render_widget(para, content);
}

fn render_res_screen(
    f: &mut Frame<'_>,
    content: Rect,
    changes: &PackageChanges,
    active_tab: usize,
    cursor: usize,
) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = content.layout(&layout);

    let tabs = Tabs::new(CATEGORIES)
        .style(Color::White)
        .highlight_style(Style::default().blue().on_black().bold())
        .select(active_tab)
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

    let mut list_state = ListState::default().with_selected(Some(cursor));
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

fn render_footer(f: &mut Frame<'_>, area: ratatui::layout::Rect, screen: &Screen) {
    let mut spinner_idx: Option<&usize> = None;
    let footer = match screen {
        Screen::SelectGenerations { .. } => Some(Tabs::new([
            "Move: (↑/↓/j/k)",
            "Pick: (Enter)",
            "Diff: (Space)",
            "Quit: (Esc)",
        ])),
        Screen::Diffing { spin_idx, .. } => {
            spinner_idx = Some(spin_idx);
            None
        }
        Screen::Results { .. } => Some(Tabs::new([
            "Switch: (Tab/Shift-Tab/0-5)",
            "Move: (↑/↓/j/k)",
            "Quit: (Esc)",
        ])),
    };

    match footer {
        Some(ft) => {
            let foot = ft
                .style(Style::default().fg(Color::DarkGray))
                .divider(symbols::DOT)
                .select(None)
                .padding(" ", " ");
            f.render_widget(foot, area);
        }
        None => {
            let frame_char = SPINNER_FRAMES[spinner_idx.unwrap() % SPINNER_FRAMES.len()];
            let para = Paragraph::new(format!("{frame_char}"))
                .style(Style::default().fg(Color::DarkGray).bold())
                .centered();
            f.render_widget(para, area);
        }
    }
}
