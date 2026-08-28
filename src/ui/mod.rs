pub mod app;
pub mod event;
pub mod render;

pub use app::{DiffApp, Screen};
pub use event::handle_events;
pub use render::draw_ui;

pub const CATEGORIES: [&str; 6] = [
    "[0] Kernel",
    "[1] Updated",
    "[2] Removed",
    "[3] Added",
    "[4] Rebuilt",
    "[5] Other",
];
