use std::fmt::Display;

pub enum PackageChangeType {
    Kernel,
    Updated,
    Removed,
    Added,
    Rebuilt,
    Other,
}

#[derive(Debug)]
pub struct PackageChange {
    pub pkg_name: String,
    pub old_ver: Vec<String>,
    pub new_ver: Vec<String>,
    pub size_delta: Option<Size>,
}

#[derive(Debug)]
pub struct Size {
    pub delta: f64,
    pub unit: String,
}

fn is_kernel_package(name: &str) -> bool {
    matches!(
        name,
        "linux"
            | "linux-zen"
            | "linux-headers"
            | "linux-headers-static"
            | "initrd-linux-zen"
            | "linux-firmware"
    )
}

impl PackageChange {
    pub fn classify_package(&self) -> PackageChangeType {
        if is_kernel_package(&self.pkg_name) {
            PackageChangeType::Kernel
        } else if !self.old_ver.is_empty() && !self.new_ver.is_empty() {
            PackageChangeType::Updated
        } else if !self.old_ver.is_empty() {
            PackageChangeType::Removed
        } else if !self.new_ver.is_empty() {
            PackageChangeType::Added
        } else if self.size_delta.is_some() {
            PackageChangeType::Rebuilt
        } else {
            PackageChangeType::Other
        }
    }
}

impl Display for PackageChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pkg_name)?;

        if !self.old_ver.is_empty() {
            write!(f, ": ")?;
            for (i, ver) in self.old_ver.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{ver}")?;
            }
        }

        if !self.new_ver.is_empty() {
            if self.old_ver.is_empty() {
                write!(f, ": ")?;
            } else {
                write!(f, " -> ")?;
            }
            for (i, ver) in self.new_ver.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{ver}")?;
            }
        }

        if self.old_ver.is_empty() && self.new_ver.is_empty() {
            write!(f, ": ")?;
        }
        if let Some(sd) = &self.size_delta
            && sd.delta != 0.0
        {
            if self.old_ver.is_empty() && self.new_ver.is_empty() {
                write!(f, "{sd}")?;
            } else {
                write!(f, ", {sd}")?;
            }
        }
        Ok(())
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.delta, self.unit)
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            delta: 0.0,
            unit: String::new(),
        }
    }
}
