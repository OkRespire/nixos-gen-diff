use anyhow::{Context, Result};
use strip_ansi_escapes::strip_str;

use crate::model::{PackageChange, Size};

/// Parses a line from the nix store diff-closures and turns it into a PackageChange type.
/// The lines can be:
/// name: old_pkg_ver → new_pkg_ver, +/- size Unit (B, KiB, GiB, etc)
/// name: old_pkg_ver2, old_pkg_ver2 → new_pkg_ver1, new_pkg_ver2, +/- Size Unit (B, KiB, GiB, etc)
/// name: new_pkg_ver1, +/- Size Unit (B, KiB, GiB, etc)
/// name: +/- Size Unit (B, KiB, GiB, etc)
/// extra.targets: ε → ∅
/// name:
/// There may be more that is missing but these are the ones that I have noticed
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_no_arrow_rebuild_only() -> Result<()> {
        let result = parse_packages("7zz: 2.8 MiB")?;
        assert_eq!(result.pkg_name, "7zz");
        assert!(result.old_ver.is_empty());
        assert!(result.new_ver.is_empty());
        let size = result.size_delta.expect("expected a size delta");
        assert_eq!(size.delta, 2.8);
        assert_eq!(size.unit, "MiB");
        Ok(())
    }

    #[test]
    fn parses_single_version_both_sides_with_size() -> Result<()> {
        let result = parse_packages("thing: 0.1 → 0.2, 245.0 KiB")?;
        assert_eq!(result.pkg_name, "thing");
        assert_eq!(result.old_ver, vec!["0.1"]);
        assert_eq!(result.new_ver, vec!["0.2"]);
        let size = result.size_delta.expect("expected a size delta");
        assert_eq!(size.delta, 245.0);
        assert_eq!(size.unit, "KiB");
        Ok(())
    }

    #[test]
    fn parses_multi_version_both_sides_with_size() -> Result<()> {
        let result = parse_packages("bottles: 65.4, 65.4-fhsenv → 64.1, 64.1-fhsenv, -188.0 KiB")?;
        assert_eq!(result.pkg_name, "bottles");
        assert_eq!(result.old_ver, vec!["65.4", "65.4-fhsenv"]);
        assert_eq!(result.new_ver, vec!["64.1", "64.1-fhsenv"]);
        let size = result.size_delta.expect("expected a size delta");
        assert_eq!(size.delta, -188.0);
        assert_eq!(size.unit, "KiB");
        Ok(())
    }

    #[test]
    fn parses_epsilon_and_empty_set_placeholders_as_empty() -> Result<()> {
        let result = parse_packages("extra.targets: ε → ∅")?;
        assert_eq!(result.pkg_name, "extra.targets");
        assert!(result.old_ver.is_empty());
        assert!(result.new_ver.is_empty());
        assert!(result.size_delta.is_none());
        Ok(())
    }
}
