use ratatui::DefaultTerminal;
use std::io;

pub fn setup<F>(app: F) -> io::Result<()>
where
    F: FnOnce(&mut DefaultTerminal) -> io::Result<()>,
{
    ratatui::run(app)
}
