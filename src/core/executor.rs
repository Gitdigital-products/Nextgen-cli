use crate::context::EnvironmentContext;
use std::process::Command;
use anyhow::Result;

// This function executes a shell command and returns its output
pub fn execute_shell_command(cmd: &str, args: &[&str], ctx: &EnvironmentContext) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .current_dir(&ctx.current_dir) // Run it in the user's current directory!
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Command failed: {}", error_msg))
    }
}