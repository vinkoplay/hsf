use std::fs;
use std::net::IpAddr;
use std::path::{PathBuf, Path};
use std::env;
use std::ffi::CString;
use libc::{uid_t, gid_t, setuid, setgid, initgroups, getuid};

pub const HOSTS_PATH: &str = "/etc/hosts";
pub const HOSTNAME_PATH: &str = "/etc/hostname";

macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

/// Drops priviliges from sudo to normal user
pub fn drop_privileges() -> Result<(), Box<dyn std::error::Error>> {
    if unsafe { getuid() } != 0 {
        return Ok(());
    }

    let uid: uid_t = env::var("SUDO_UID")?.parse()?;
    let gid: gid_t = env::var("SUDO_GID")?.parse()?;
    let user = env::var("SUDO_USER")?;
    let c_user = CString::new(user)?;

    unsafe {
        // 3. Сбрасываем доп. группы
        if initgroups(c_user.as_ptr(), gid) != 0 {
            return Err("initgroups failed".into());
        }

        // 4. Меняем группу
        if setgid(gid) != 0 {
            return Err("setgid failed".into());
        }

        // 5. Меняем пользователя
        if setuid(uid) != 0 {
            return Err("setuid failed".into());
        }
    }

    Ok(())
}

/// Return paths to user config
pub fn get_xdg_config_home() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(path);
    }

    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config")
}


/// Write the temp file near and after rename to replace original
pub fn atomic_write(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", path);
    
    fs::write(&temp_path, content)?;
    
    if let Err(e) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(e.into());
    }
    
    Ok(())
}

/// Count rules in hosts, need it content
pub fn count_rules(content: &str) -> usize {
    content.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .count()
}

/// Function write to /etc/hosts, what is got
pub fn write_hosts(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    atomic_write(HOSTS_PATH, content)?;
    println!(
        "Written ({} bytes, {} chars, {} lines)",
        content.len(),
        content.chars().count(),
        content.lines().count()
    );

    Ok(())
}

/// Function read /etc/hosts file and return content
pub fn read_hosts() -> Result<String, Box<dyn std::error::Error>> {
    let string: String = fs::read_to_string(HOSTS_PATH)?;
    Ok(string)
}

/// Reads /etc/hostname for hostname and returning it
pub fn get_hostname() -> Result<String, Box<dyn std::error::Error>> {
    let hostname = fs::read_to_string(HOSTNAME_PATH)?.trim().to_string();
    Ok(hostname)
}

/// Write to /etc/hostname, what is got
pub fn set_hostname(hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    atomic_write(HOSTNAME_PATH, hostname)?;
    println!(
        "Written ({} bytes, {} chars, {} lines)",
        hostname.len(),
        hostname.chars().count(),
        hostname.lines().count()
    );
    Ok(())
}

/// Checks if the hostname contains only valid characters (a-z, 0-9, -)
pub fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 63 {
        return false;
    }

    for (i, c) in hostname.chars().enumerate() {
        if c.is_ascii_alphanumeric() {
            continue;
        }
        if c == '-' {
            if i == 0 || i == hostname.len() - 1 {
                return false;
            }
            continue;
        }
        return false;
    }
    true
}

/// Checks ip is correct or no
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

/// Check domain is correct or no
pub fn is_valid_domain(domain: &str) -> bool {
    let domain = domain.trim();
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }
    // Запрещаем точки и дефисы по краям + двойные точки
    if domain.starts_with('-') || domain.ends_with('-') 
       || domain.starts_with('.') || domain.ends_with('.') 
       || domain.contains("..") { // <-- Добавили проверку на ".."
        return false;
    }
    domain.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '.')
}

/// Trims protocol and path from the URL
fn trim_protocol(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed = url.trim();
    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        let result = trimmed.trim_start_matches("https://")
         .trim_start_matches("http://")
         .split('/')
         .next().unwrap_or("");
        return Ok(result.to_string());
    }
    Err("Incorrect protocol".into())
}

/// Checks if IP is correct (supports http/https)
pub fn is_valid_ip_with_protocol(url: &str) -> bool {
    let trimmed_ip = trim_protocol(url);
    if trimmed_ip.is_err() {return false;}
    
    trimmed_ip.unwrap().parse::<IpAddr>().is_ok()
}

/// Checks if domain is correct (supports http/https)
pub fn is_valid_domain_with_protocol(url: &str) -> bool {
    let trimmed_domain = trim_protocol(url);
    if trimmed_domain.is_err() {return false;}
    let domain = trimmed_domain.unwrap();
    
    
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }
    if domain.starts_with('-') || domain.ends_with('-') 
       || domain.starts_with('.') || domain.ends_with('.') 
       || domain.contains("..") {
        return false;
    }
    domain.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '.')
}

/// Checks filename for correct
pub fn _is_valid_filename(name: &str) -> bool {
    if name.is_empty() || name.len() > 255 {
        return false;
    }

    if name == "." || name == ".." {
        return false;
    }

    name.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'
    })
}

/// Checks valid content of hosts file
pub fn is_valid_hosts_content(content: &str) -> bool {
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.split('#').next().unwrap_or("").trim();
        
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.split_whitespace();

        let ip_part = match parts.next() {
            Some(p) => p.split('%').next().unwrap_or(p),
            None => continue,
        };

        let clean_ip = ip_part.trim_matches(|c: char| !c.is_ascii_graphic());
        if clean_ip.parse::<std::net::IpAddr>().is_err() {
            println!("Error on line {}.", idx);
            debug_println!("Raw bytes of IP: {:?}", ip_part.as_bytes());
            return false;
        }

        if parts.next().is_none() {
            println!("Error on line {}.", idx);
            return false;
        }
    }

    true
}

/// Programm was started on root rights or no
pub fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}