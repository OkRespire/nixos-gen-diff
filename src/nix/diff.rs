use anyhow::{Context, Result};
use strip_ansi_escapes::strip_str;

use crate::model::{PackageChange, Size};

pub fn parse_packages(input: &str) -> Result<PackageChange> {
    let input = strip_str(input);
    let fields: Vec<&str> = input
        .split_whitespace()
        .map(|s| s.trim_end_matches(','))
        .collect();

    // eprintln!("DEBUG fields: {:?}", fields);
    let name = fields[0].trim_end_matches(':').to_string();
    let Some(arrow_idx) = fields.iter().position(|f| *f == "→") else {
        let delta: f64 = fields[1]
            .parse()
            .context("This field cannot be parsed into a float64")?;
        let size = Some(Size {
            delta,
            unit: fields[2].to_string(),
        });
        return Ok(PackageChange {
            pkg_name: name,
            old_ver: Vec::new(),
            new_ver: Vec::new(),
            size_delta: size,
        });
    };

    let mut new_end = fields.len();
    let mut size_delta = None;

    if fields.len() >= arrow_idx + 3 {
        let delta_idx = fields.len() - 2;
        if let Ok(delta) = fields[delta_idx].parse::<f64>() {
            size_delta = Some(Size {
                delta,
                unit: fields[delta_idx + 1].to_string(),
            });
            new_end = delta_idx;
        }
    }

    let mut old_ver: Vec<String> = fields[1..arrow_idx]
        .iter()
        .map(|s| s.trim_end_matches(',').to_string())
        .collect();

    let mut new_ver: Vec<String> = fields[arrow_idx + 1..new_end]
        .iter()
        .map(|s| s.trim_end_matches(',').to_string())
        .collect();

    if old_ver.len() == 1 && (old_ver[0] == "∅" || old_ver[0] == "ε") {
        old_ver.clear();
    }

    if new_ver.len() == 1 && (new_ver[0] == "∅" || new_ver[0] == "ε") {
        new_ver.clear();
    }

    Ok(PackageChange {
        pkg_name: name,
        old_ver,
        new_ver,
        size_delta,
    })
}
