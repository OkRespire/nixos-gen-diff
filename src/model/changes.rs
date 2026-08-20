use std::fmt::Display;

use super::{PackageChange, PackageChangeType};

#[derive(Debug, Default)]
pub struct PackageChanges {
    pub kernel: Vec<PackageChange>,
    pub updated: Vec<PackageChange>,
    pub removed: Vec<PackageChange>,
    pub added: Vec<PackageChange>,
    pub rebuilt: Vec<PackageChange>,
    pub other: Vec<PackageChange>,
}

impl PackageChanges {
    pub fn from_packages(packages: Vec<PackageChange>) -> Self {
        let mut changes = Self::default();

        for package in packages {
            match package.classify_package() {
                PackageChangeType::Kernel => changes.kernel.push(package),
                PackageChangeType::Updated => changes.updated.push(package),
                PackageChangeType::Removed => changes.removed.push(package),
                PackageChangeType::Added => changes.added.push(package),
                PackageChangeType::Rebuilt => changes.rebuilt.push(package),
                PackageChangeType::Other => changes.other.push(package),
            }
        }

        changes
    }
}

impl Display for PackageChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- Package Update Summary ---")?;
        if !self.kernel.is_empty() {
            writeln!(f, "[Kernel Updates]")?;
            print_vec(f, &self.kernel)?;
        }
        if !self.updated.is_empty() {
            writeln!(f, "[Updated Packages]")?;
            print_vec(f, &self.updated)?;
        }
        if !self.removed.is_empty() {
            writeln!(f, "[Removed Packages]")?;
            print_vec(f, &self.removed)?;
        }
        if !self.added.is_empty() {
            writeln!(f, "[Added Packages]")?;
            print_vec(f, &self.added)?;
        }
        if !self.rebuilt.is_empty() {
            writeln!(f, "[Rebuilt Packages]")?;
            print_vec(f, &self.rebuilt)?;
        }
        if !self.other.is_empty() {
            writeln!(f, "[Other Changes]")?;
            print_vec(f, &self.other)?;
        }
        Ok(())
    }
}

fn print_vec(f: &mut std::fmt::Formatter<'_>, vec: &[PackageChange]) -> std::fmt::Result {
    for v in vec {
        writeln!(f, "{}", v)?
    }
    writeln!(f, "\n")?;
    Ok(())
}
