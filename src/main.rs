use std::env;

mod backup;
mod sysfiles;
mod utils;
mod preset;
mod preproccesor;

pub const VERSION: &str = "1.0.0";

#[cfg(not(target_os = "linux"))]
compile_error!("This project is linux only");

/// Print help info
fn help() -> Result<(), Box<dyn std::error::Error>> {
    println!(include_str!("../assets/help.txt"));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let empty_string = String::new();

    if args.len() < 2 {
        println!("hsf help - for help info");
        println!("nothing to do");
        return Ok(());
    }

    match args[1].as_str() {
        "help" | "h" | "--help" | "-h" => {
            help()?;
        }
        "version" | "v" | "--version" | "-v" => {
            println!("HSF Version {}", VERSION);
        }
        "clear" | "c" | "--clear" | "-c" => {
            utils::clear()?;
        }
        "reset" | "r" | "--reset" | "-r" | "base" | "--base" | "ba" | "-ba"=> {
            utils::reset()?;
        }
        "backup" | "b" | "--backup" | "-b" => {
            let action = args.get(2).unwrap_or(&empty_string);
            let name = args.get(3).unwrap_or(&empty_string);
            if action.is_empty() {
                return Err("No action for backup.".into());
            }

            backup::parse_backup(action, name)?;
        }
        "info" | "i" | "--info" | "-i" | "stat" | "zstat" => {
            utils::info()?;
        }
        "check" | "ch" | "ck" | "--check" | "-ch" | "-ck" => {
            utils::check(args.get(2).unwrap_or(&empty_string))?;
        }
        "add" | "a" | "--add" | "-a" => {
            let ip = args.get(2).unwrap_or(&empty_string);
            let domain = args.get(3).unwrap_or(&empty_string);
            utils::add(ip, domain)?;
        }
        "remove" | "rem" | "delete" | "d" | "rm" | "--remove" | "-rem" | "--delete" | "-d" | "-rm" => {
            let domain = args.get(2).unwrap_or(&empty_string);
            utils::remove(domain)?;
        } 
        "preset" | "p" | "--preset" | "-p" => {
            let action = args.get(2).unwrap_or(&empty_string);
            let arg = args.get(3).unwrap_or(&empty_string);
            if action.is_empty() {
                return Err("No args for action/preset name.".into());
            }
            preset::parse_preset(action, arg)?;
        }
        _ => {
            println!("hsf help - for help info");
            println!("nothing to do");
        }
    }

    Ok(())
}
