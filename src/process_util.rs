use std::str::FromStr;

use anyhow::{Context, bail};
use sysinfo::{Pid, System};
use tokio::process::Command;

pub async fn get_tmux_pid(session: &str) -> anyhow::Result<String> {
    let target = format!("{session}:0.0");

    let output = Command::new("tmux")
        .args(["display-message", "-p", "-t", &target, "#{pane_pid}"])
        .output()
        .await
        .with_context(|| format!("failed to get tmux pane pid for {target}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("tmux failed for target {target}: {stderr}");
    }

    let pid = String::from_utf8(output.stdout)
        .context("tmux returned non-UTF-8 pane pid")?
        .trim()
        .to_string();

    Ok(pid)
}

pub async fn get_memory_usage(pid: &str) -> anyhow::Result<String> {
    let mut system = System::new_all();
    system.refresh_all();

    let pid = Pid::from_str(pid)?;
    let process = &system.processes()[&pid];

    let memory = process.memory();

    let result = if memory < (1_u64 << 30) {
        let mb = memory as f64 / (1_u64 << 20) as f64;
        format!("{mb:.2} MB")
    } else {
        let gb = memory as f64 / (1_u64 << 30) as f64;
        format!("{gb:.2} GB")
    };

    Ok(result)
}
