use std::path::Path;
use std::process::{Command, Stdio};

pub async fn run_script(script_path: &Path) -> Result<(), std::io::Error> {
    let extension = script_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let mut command = if cfg!(target_os = "windows") {
        match extension {
            "bat" | "cmd" => {
                let mut cmd = Command::new("cmd");
                // TODO hardcoded on /C change to be dynamic 
                cmd.arg("/C").arg(script_path);
                cmd
            }
            "sh" => {
                let mut cmd = Command::new("bash");
                cmd.arg(script_path);
                cmd
            }
            _ => {
                let cmd = Command::new(script_path);
                cmd
            }
        }
    } else {
        match extension {
            "sh" => {
                let mut cmd = Command::new("sh");
                cmd.arg(script_path);
                cmd
            }
            _ => {
                let cmd = Command::new(script_path);
                cmd
            }
        }
    };

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    
    let mut child = command.spawn()?;

    tokio::task::spawn_blocking(move || {
        let _ = child.wait();
    });

    Ok(())
}
