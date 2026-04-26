use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::{sysfiles, utils, preproccesor};

pub const MAX_SIZE: u64 = 7 * 1024 * 1024; // 7 MB

/// Finds a preset file in xdgconfighome/hsf/presets, /etc/hsf/presets or in path
fn find_preset_file(input_path: &str) -> Result<PathBuf, Box<dyn  std::error::Error>> {
    let paths = [
        sysfiles::get_xdg_config_home().join("hsf/presets").join(input_path),
        PathBuf::from("/etc/hsf/presets").join(input_path),
        PathBuf::from(input_path),
        env::current_dir()?.join(input_path)
    ];
    
    for path in paths.iter() {
        if path.exists() && path.is_file() {
            return Ok(path.to_path_buf());
        }
    }

    Err(format!("Couldn't find preset file {}", input_path).into())
}

/// Entry point of presets
pub fn parse_preset(action: &str, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    if sysfiles::is_root() {
        let etc_presets_path = PathBuf::from("/etc/hsf/presets");
        if !etc_presets_path.exists() {
            fs::create_dir_all(&etc_presets_path)?;
        }
        let base_preset_path = etc_presets_path.join("base");
        if !base_preset_path.exists() {
            fs::write(&base_preset_path, include_str!("../assets/base.hosts"))?;
        }
    }

    match action {
        "load" | "from" | "apply" | "--load" | "--from" | "--apply" | "lo" | "f" | "ld" | "a" | "-lo" | "-f" | "-ld" | "-a"  => {
            load_preset(&arg)?;
        }
        "list" | "l" | "--list" | "-l" | "search" | "s" | "--search" | "-s" => {
            list_preset(&arg)?;
        }
        "format" | "fo" | "fr" | "--format" | "-fo" | "-fr" | "result" | "--result" | "r" | "-r" => {
            let path = PathBuf::from(&arg);
            format_preset(&path)?;
        }
        _ => {
            load_preset(&action)?;
        }
    }
    Ok(())
}

pub fn format_preset(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = format_preset_run(path) {
        println!("E: {}", e);
        std::process::exit(1)
    }
    std::process::exit(0);
}

pub fn format_preset_run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() || !path.is_file() {
        return Err("incorrect path or this is directory".into())
    }

    let hostname = utils::get_right_hostname()?;
    let worked_file = preproccesor::process_all_includes(path)?;

    sysfiles::drop_privileges()?;
    
    let preprocessed_file = preproccesor::process_all_requires(&worked_file)?;
    let processed_file = preprocessed_file.replace("%HOSTNAME%", &hostname);

    if processed_file.len() > MAX_SIZE as usize {
        return Err("Content limit exceeded".into());
    }

    if !sysfiles::is_valid_hosts_content(&processed_file) {
        return Err("Incorrect hosts format file".into());
    }

    println!("{}", processed_file);

    Ok(())
}

/// Loads preset to hosts file
pub fn load_preset(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = find_preset_file(name)?;
    let current_executable = std::env::current_exe()?;
    
    let mut output = String::new();
    let mut child = Command::new("timeout")
        .arg("7s")
        .arg(&current_executable)
        .arg("preset")
        .arg("format")
        .arg(path.as_os_str())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut output)?;
    }

    let status = child.wait()?;
    let has_errors = output.starts_with("E:") || output.starts_with("e:");
    let code = status.code().ok_or("Could't get code of children")?;

    if code != 0 || has_errors {
        if code == 124 {
            println!("Timeout for formating preset")
        } else {
            println!("Error: {}", output)
        }
        
    } else {
        if output.len() <= MAX_SIZE as usize && sysfiles::is_valid_hosts_content(&output) {
            sysfiles::write_hosts(&output)?;
        } else {
            return Err("Incorrect hosts format file on input".into());
        }
    }

    Ok(())
}

/// Print list of presets
pub fn list_preset(search: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = [
        sysfiles::get_xdg_config_home().join("hsf/presets"),
        PathBuf::from("/etc/hsf/presets"),
    ];

    for path in paths.iter() {
        if !path.exists() || !path.is_dir() { continue; }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();

            if search.is_empty() || name.contains(search) {
                if entry.path().is_file() {
                    println!("{}", name);
                }
            }
        }
    }

    Ok(())
}