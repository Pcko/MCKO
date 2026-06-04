use std::{collections::HashMap, fs, path::Path, process::Command};

use anyhow::{Context, Result, ensure};

use crate::app_config::AppConfig;

pub async fn run_setup(config: &AppConfig) -> Result<()> {
    check_server_location(config)?;
    accept_eula(config)?;
    configure_server_properties(config)?;
    check_java_install()?;

    Ok(())
}

fn check_server_location(config: &AppConfig) -> Result<()> {
    let dir = Path::new(&config.mc_server_dir);
    let jar_path = dir.join(dotenv::var("MC_SERVER_JAR").expect("MC_SERVER_JAR must be set"));

    ensure!(
        dir.exists(),
        "ERR Server directory does not exist: {}",
        dir.display()
    );

    ensure!(
        dir.is_dir(),
        "ERR Server path exists but is not a directory: {}",
        dir.display()
    );

    println!("OK Server directory exists");

    ensure!(
        jar_path.exists(),
        "ERR Server jar does not exist: {}",
        jar_path.display()
    );

    ensure!(
        jar_path.is_file(),
        "ERR Server jar path exists but is not a file: {}",
        jar_path.display()
    );

    println!("OK Server jar exists");
    Ok(())
}

fn accept_eula(config: &AppConfig) -> Result<()> {
    let eula_path = Path::new(&config.mc_server_dir).join("eula.txt");

    fs::write(&eula_path, "eula=true\n")
        .with_context(|| format!("failed to write {}", eula_path.display()))?;

    println!("CHANGE eula.txt: eula=true");
    Ok(())
}

fn configure_server_properties(config: &AppConfig) -> Result<()> {
    //TODO check for rcon enabled in .env
    let managed = HashMap::from([
        ("server-port", config.mc_port.to_string()),
        ("enable-rcon", "true".to_string()),
        ("rcon.port", config.rcon_port.to_string()),
        ("rcon.password", config.rcon_password.clone()),
    ]);

    let properties_path = Path::new(&config.mc_server_dir).join("server.properties");
    let content = fs::read_to_string(&properties_path)?;
    let mut new_content: Vec<String> = vec![];
    let mut seen = HashMap::new(); // Track keys

    for line in content.lines() {
        if line.starts_with("#") || !line.contains('=') {
            new_content.push(line.to_string());
            continue;
        }

        // if not a (single) key value pair skip it
        let Some((key, _value)) = line.split_once('=') else {
            new_content.push(line.to_string());
            continue;
        };

        // if target key value pair change it else keep it
        if let Some(value) = managed.get(key) {
            let new_line = format!("{key}={value}");

            println!("CHANGE server.properties: {new_line}");
            new_content.push(new_line);
            seen.insert(key, true);
        } else {
            new_content.push(line.to_string());
        }
    }

    // if field hasnt existed until now add them
    for (key, value) in &managed {
        if !seen.contains_key(key) {
            let new_line = format!("{key}={value}");

            println!("CHANGE server.properties: {new_line}");
            new_content.push(new_line);
        }
    }

    fs::write(&properties_path, new_content.join("\n") + "\n")
        .with_context(|| format!("failed to write {}", properties_path.display()))?;

    Ok(())
}

fn check_command_exists(command : &str) -> Result<()> {
    ensure!(
        which::which(command).is_ok(),
        "ERR Java is not installed."
    );
    
    println!("OK {command} found");
    Ok(())
}

fn check_java_install() -> Result<()>{
    check_command_exists("java")?;

    let output = Command::new("java")
    .arg("-version")
    .output()?;

    let version = String::from_utf8_lossy(&output.stdout);
    println!(" {}", version.trim());

    Ok(())
}