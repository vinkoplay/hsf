use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, Read};

use crate::{preset, sysfiles};

/// Preprocces the #include <> importing other files
pub fn process_all_includes(path: &Path) -> Result<String, Box<dyn  std::error::Error>> {
    let mut visited = HashSet::new();
    recursive_include(path, &mut visited)
}

fn recursive_include(path: &Path, visited: &mut HashSet<PathBuf>) -> Result<String, Box<dyn  std::error::Error>> {
    let canonical_path = fs::canonicalize(path)?;
    if visited.contains(&canonical_path) {
        return Ok("".into());
    }

    visited.insert(canonical_path.clone());

    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("#include <") && trimmed.ends_with(">") {
            let include_name = &trimmed[10..trimmed.len() - 1];
            let mut include_path = path.parent().unwrap_or(Path::new(".")).to_path_buf();
            include_path.push(include_name);

            match recursive_include(&include_path, visited) {
                Ok(content) => output.push_str(&content),
                Err(e) => eprintln!("Warning: could not include {:?}: {}", include_path, e),
            }
        } else {
            output.push_str(&line);
            output.push('\n');
        }
    }

    Ok(output)
}

/// Preprocces the #require <> gets from the sites
pub fn process_all_requires(content: &str) -> Result<String, Box<dyn  std::error::Error>> {
    let mut visited: HashSet<String> = HashSet::new();
    recursive_require(content, &mut visited)
}

fn get_form_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = String::new();
    ureq::get(url)
        .call()?
        .into_body()
        .into_reader()
        .take(preset::MAX_SIZE)
        .read_to_string(&mut body)?;
    Ok(body)
}

fn recursive_require(content: &str, visited: &mut HashSet<String>) -> Result<String, Box<dyn  std::error::Error>> {
    let mut output = String::new();
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#require <") && trimmed.ends_with(">") {
            let require_name = &trimmed[10..trimmed.len() -1];
            if sysfiles::is_valid_domain_with_protocol(require_name) || sysfiles::is_valid_ip_with_protocol(require_name) {
                let require_string = require_name.to_string();
                if !visited.contains(&require_string) {
                    visited.insert(require_string);
                    let downloaded = get_form_url(require_name)?;
                    let result = recursive_require(&downloaded, visited)?;
                    output.push_str(&result);
                    output.push('\n');
                }
            }
        } else {
            output.push_str(&line);
            output.push('\n');
        }
    }
    Ok(output)
}