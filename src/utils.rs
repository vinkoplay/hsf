use std::io;
use std::io::Write;
use std::fs;
use std::path::{PathBuf};

use crate::sysfiles;

pub const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

/// Return date format from config
pub fn get_date_format() -> String {
    let search_paths = [
        sysfiles::get_xdg_config_home().join("hsf/time_format"),
        PathBuf::from("/etc/hsf/time_format"),
    ];

    for path in &search_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    DEFAULT_TIME_FORMAT.to_string()
}


/// Function input from python: got prompt and print it and ask user to write in console, after returns his input
pub fn input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end_matches('\n').trim_end_matches('\r').to_string())
}

/// Gets rights hostname with asking the user
pub fn get_right_hostname() -> Result<String, Box<dyn std::error::Error>> {
    let mut hostname = String::new();
    let file_hostname = sysfiles::get_hostname().unwrap_or(String::new());
    let mut dirty = false;
    let mut first = true;

    if file_hostname.is_empty() {
        while hostname.is_empty() || !sysfiles::is_valid_hostname(&hostname) {
            if !hostname.is_empty() && !first {
                println!("Uncorrect hostname ( a-z, 0-9, - )");
            }
            hostname = input("Enter hostname: ")?;
            first = false;
        }
        dirty = true;
    } else {
        loop {
            hostname = input("Enter hostname (leave empty to use /etc/hostname): ")?;

            if hostname.is_empty() || sysfiles::is_valid_hostname(&hostname) {
                break;
            }

            println!("Incorrect hostname ( a-z, 0-9, - )");
        }

        if hostname.is_empty() {
            hostname = file_hostname;
        } else {
            dirty = true;
        }
    }

    if dirty {
        let want = input(&format!(
            "Do you wand edit /etc/hostname to {}? [y/N]",
            &hostname
        ))?;
        if want.is_empty() {
            sysfiles::set_hostname(&hostname)?;
        }
    }

    Ok(hostname)
}

/// Clears hosts file to empty
pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
    let user_confirm = input("Are you sure you want to completely clear /etc/hosts? [y/N]: ")?;
    if user_confirm.to_lowercase() == "y" {
        sysfiles::write_hosts("")?;
    }
    Ok(())
}

/// Writed to hosts file normal state
pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    let template = include_str!("../assets/base.hosts");
    let hostname = get_right_hostname()?;
    let content = template.replace("%HOSTNAME%", &hostname);

    sysfiles::write_hosts(&content)?;

    Ok(())
}

/// Prints short info about hosts file
pub fn info() -> Result<(), Box<dyn std::error::Error>> {
    let hosts_file = sysfiles::read_hosts()?;
    let rules_count = sysfiles::count_rules(&hosts_file);

    println!(
        " Rules: {} \n Lines: {} \n Chars: {} \n Bytes: {}",
        rules_count,
        hosts_file.lines().count(),
        hosts_file.chars().count(),
        hosts_file.len()
    );

    Ok(())
}

/// Check domain for existing and return its ip
pub fn check(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !sysfiles::is_valid_domain(&domain) {
        return Err("Domain couldn't be empty".into());
    }
    let content = sysfiles::read_hosts()?;
    for text in content.lines() {
        let trimmed = text.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {continue;}
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[1..].contains(&domain) {
            println!("{} points to {}", &domain, &parts[0]);
            return Ok(());
        }
    }
    println!("{} doesn't exist in hosts file", &domain);
    Ok(())
}

/// Adds domain to hosts
pub fn add(ip: &str, domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !sysfiles::is_valid_ip(&ip) || !sysfiles::is_valid_domain(&domain) {
        return Err("Ip or domain couldn't be empty".into())
    }
    let content = sysfiles::read_hosts()?;
    
    let mut founded = false;
    let mut is_duplicate = false;
    let mut found_line: usize = 0;
    
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {continue;}

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[1..].contains(&domain) {
            founded = true;
            found_line = i;
            if parts[0] == ip {
                is_duplicate = true;
            }
            break;
        }
    }

    if founded {
        if is_duplicate {
            println!("{} already points to {} (line: {})", &domain, &ip, found_line+1);
        } else {
            let want = input(&format!("Domain {} already exists. Replace with {}? [y/N]: ", 
            &domain, &ip))?;
            if want.to_lowercase() == "y" {
                let mut new_lines: Vec<String> = Vec::new();
                
                for (i, line) in content.lines().enumerate() {
                    if i == found_line {
                        let mut parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
                        parts.retain(|x| x != domain);

                        if parts.len() > 1 {
                            new_lines.push(parts.join(" "));
                        }
                    } else {
                        new_lines.push(line.to_string());
                    }
                }

                let mut final_content = new_lines.join("\n").trim_end().to_string();
                final_content.push_str(&format!("\n{} {}\n", &ip, &domain));

                sysfiles::write_hosts(&final_content)?;
            }
        }
    } else {
        let trimmed_content = content.trim_end();
        let content_file = format!("{}\n{} {}\n", &trimmed_content, &ip, &domain);
        sysfiles::write_hosts(&content_file)?;
    }

    Ok(())
}

/// Removes domain from hosts file
pub fn remove(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !sysfiles::is_valid_domain(&domain) {
        return Err("domain is incorrect".into());
    }

    let content = sysfiles::read_hosts()?;
    let mut new_lines: Vec<String> = Vec::new();
    let mut removed = false;

    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() || trimmed.starts_with('#') {
            new_lines.push(line.to_string());
            continue;
        }

        let mut parts: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();

        if parts.len() >= 2 && parts[1..].contains(&domain.to_string()) {
            parts.retain(|x| x != domain);
            removed = true;

            if parts.len() > 1 {
                new_lines.push(parts.join(" "));
            }
        } else {
            new_lines.push(line.to_string());
        }
    }

    if removed {
        let new_content = new_lines.join("\n") + "\n";
        sysfiles::write_hosts(&new_content)?;
    } else {
        println!("Domain {} not found", &domain);
    }

    Ok(())
}
