use crate::model::{Generation, PackageChanges};

pub struct DiffApp {
    pub screen: Screen,
    pub should_quit: bool,
}

pub enum Screen {
    SelectGenerations {
        generations: Vec<Generation>,
        cursor: usize,
        picked: Vec<u32>,
    },
    Diffing {
        old: Generation,
        new: Generation,
    },
    Results {
        changes: PackageChanges,
        active_tab: usize,
        list_cursor: usize,
    },
}
