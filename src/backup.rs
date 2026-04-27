use std::fs;
use std::path::{Path, PathBuf};
use chrono::{Local, DateTime};

use crate::{sysfiles, utils};

const BACKUP_FOLDER_PATH: &str = "/var/lib/hsf/backups";

/// Struct for storage backups
struct HostsBackup {
    create_time: i64,
    content: String,
}

impl HostsBackup {
    fn to_string(&self) -> String {
        format!("# {} \n{}", self.create_time, self.content)
    }

    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(path, self.to_string())?;
        Ok(())
    }

    fn from(raw: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (head, body) = raw.split_once('\n').unwrap_or(("", &raw));

        Ok(Self {
            create_time: head.trim_matches(|c: char| !c.is_numeric()).parse().unwrap_or(0),
            content: body.to_string(),
        })
    }

    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = fs::read_to_string(path)?;
        Self::from(&raw)
    }

}

/// Check is valid backup name or no
pub fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    
    if bytes.is_empty() || bytes.len() > 255 {
        return false;
    }

    let reserved = ["src", "target", ".", ".."];
    if reserved.contains(&name.to_lowercase().as_str()) {
        return false;
    }

    for &b in bytes {
        match b {
            0 => return false, 
            b'/' => return false, 
            b'\\' => return false,
            b':' | b'*' | b'?' | b'"' | b'<' | b'>' | b'|' => return false,
            0..=31 | 127 => return false,
            _ => continue,
        }
    }

    if name.starts_with('-') {
        return false;
    }

    true
}


/// Entry point for arguments
pub fn parse_backup(action: &str, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    match action.to_lowercase().as_str() {
        "new" | "n" | "--new" | "-n" => {
            new_backup(&arg)?;
        }
        "from" | "f" | "--from" | "-f" => {
            from_backup(&arg)?;
        }
        "del" | "d" | "delete" | "--delete" | "--del" | "-d" | "rm" => {
            del_backup(&arg)?;
        }
        "list" | "l" | "--list" | "-l" | "ls" | "-ls" => {
            list_backup(&arg)?;
        }
        "info" | "i" | "--info" | "-i" | "stat" | "zstat "=> {
            info_backup(&arg)?;
        }
        _ => return Err(format!("Action {} doesn't exist", action).into()),
    }
    Ok(())
}

/// Reutrn path to backup file with creating folders
fn get_path(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    fs::create_dir_all(BACKUP_FOLDER_PATH)?;
    Ok(PathBuf::from(BACKUP_FOLDER_PATH).join(format!("{}.hosts", name)))
}

/// Created a backup /etc/hosts file
pub fn new_backup(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_name(&name) {
        return Err("Name is incorrect".into());
    }

    let path = get_path(&name)?;
    let now = Local::now();

    if path.exists() {
            let need_rewrite = utils::input(&format!(
            "backup '{}' already exists, do you want rewrite it? [y/N]: ",
            &name
        ))?;
        if need_rewrite.to_lowercase() == "y" {
            let backup_content = HostsBackup {
                create_time: now.timestamp(),
                content: sysfiles::read_hosts()?,
            };
            backup_content.save(&path)?;
            println!("Written ( {} )", path.to_string_lossy());
        }
    } else {
        let backup_content = HostsBackup {
            create_time: now.timestamp(),
            content: sysfiles::read_hosts()?,
        };
        backup_content.save(&path)?;
        println!("Written ( {} )", path.to_string_lossy());
    }

    Ok(())
}

/// Load the backup from target
pub fn from_backup(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_name(&name) {
        return Err("Name is incorrect".into());
    }

    let path = get_path(&name)?;
    if path.exists() {
        let confirm = utils::input("Are you sure you want to replace hosts with backup? [y/N]: ")?;
        if confirm.to_lowercase() == "y" {
            let backup_content = HostsBackup::load(&path)?;
            sysfiles::write_hosts(&backup_content.content)?;
        }
    } else {
        println!("Backup {} doesn't exist", &name);
    }

    Ok(())
}

/// Delete the target backup
pub fn del_backup(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_name(&name) {
        return Err("Name is incorrect".into());
    }

    let path = get_path(&name)?;
    if path.exists() {
        let confirm = utils::input(&format!(
            "Are you sure you want to delete {} backup? [y/N]: ",
            name
        ))?;
        if confirm.to_lowercase() == "y" {
            fs::remove_file(path)?;
        }
    } else {
        println!("Backup {} doesn't exist", &name);
    }

    Ok(())
}

/// Print list of backups
pub fn list_backup(search: &str) -> Result<(), Box<dyn std::error::Error>> {
    for current_file in fs::read_dir(BACKUP_FOLDER_PATH)? {
        let backup_file = current_file?.path();
        if backup_file.is_file() && backup_file.extension().is_some_and(|ext| ext == "hosts") {
            if let Some(backup_name) = backup_file.file_stem().and_then(|s| s.to_str()) {
                if search.is_empty()
                    || backup_name
                        .to_lowercase()
                        .contains(search.to_lowercase().as_str())
                {
                    println!("{}", backup_name);
                }
            }
        }
    }

    Ok(())
}

/// Print statistics about backup
pub fn info_backup(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_name(&name) {
        return Err("Name is incorrect".into());
    }

    let path = get_path(&name)?;
    if path.exists() {
        let file_content = fs::read_to_string(&path)?;
        let backup_content = HostsBackup::from(&file_content)?;
        let datetime = DateTime::from_timestamp_secs(backup_content.create_time).unwrap_or_default().with_timezone(&Local).format(&utils::get_date_format()).to_string();

        println!("Created: {} \nRules: {}\nStorage: (Lines: {}, Chars: {}, Bytes: {}) \nContent: (Lines: {}, Chars: {}, Bytes: {})",
                datetime, sysfiles::count_rules(&file_content), file_content.lines().count(), file_content.chars().count(), file_content.len(), backup_content.content.lines().count(), backup_content.content.chars().count(), backup_content.content.len())
    } else {
        println!("Backup {} doesn't exist", &name);
    }
    Ok(())
}
