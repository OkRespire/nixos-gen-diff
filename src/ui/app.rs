use std::sync::mpsc;

use crate::{
    model::{Generation, PackageChange, PackageChanges},
    nix::generation::diff_generations,
};
use anyhow::Result;

pub struct DiffApp {
    pub screen: Screen,
    pub should_quit: bool,
}

pub enum Screen {
    SelectGenerations {
        generations: Vec<Generation>,
        cursor: usize,
        picked: Vec<Generation>,
    },
    Diffing {
        old: Generation,
        new: Generation,
        rx: mpsc::Receiver<Result<Vec<PackageChange>>>,
    },
    Results {
        changes: PackageChanges,
        active_tab: usize,
        cursor: usize,
    },
}

pub fn advance(app: &mut DiffApp) -> Result<()> {
    if let Screen::Diffing { rx, .. } = &app.screen {
        match rx.try_recv() {
            Ok(Ok(pkgs)) => {
                let changes = PackageChanges::from_packages(pkgs);
                app.screen = Screen::Results {
                    changes,
                    active_tab: 0,
                    cursor: 0,
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err(anyhow::anyhow!(
                    "diff thread ended without sending a result"
                ));
            }
        }
    }
    Ok(())
}
