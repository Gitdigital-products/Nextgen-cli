use serde::Serialize;
use std::path::PathBuf;
use which::which;

// The main struct that holds all gathered context
#[derive(Debug, Serialize, Clone)]
pub struct EnvironmentContext {
    pub current_dir: PathBuf,
    pub current_dir_str: String,
    pub project_type: Option<String>,
    pub git_branch: Option<String>,
    pub git_status: Option<String>, // simplified status, e.g., "clean", "dirty"
    pub os: String,
    pub shell: String,
    pub available_tools: AvailableTools,
}

// A struct to track which common tools are available on the system
#[derive(Debug, Serialize, Clone)]
pub struct AvailableTools {
    pub git: bool,
    pub node: bool,
    pub npm: bool,
    pub cargo: bool,
    pub python: bool,
    pub docker: bool,
}

impl Default for AvailableTools {
    fn default() -> Self {
        Self {
            git: false,
            node: false,
            npm: false,
            cargo: false,
            python: false,
            docker: false,
        }
    }
}

pub async fn gather() -> EnvironmentContext {
    let current_dir = std::env::current_dir().unwrap();

    // Get project type based on heuristics (e.g., presence of Cargo.toml, package.json)
    let project_type = detect_project_type(&current_dir);

    // Get Git information
    let (git_branch, git_status) = get_git_info(&current_dir);

    // Check for available tools
    let available_tools = check_available_tools();

    EnvironmentContext {
        current_dir: current_dir.clone(),
        current_dir_str: current_dir.to_string_lossy().to_string(),
        project_type,
        git_branch,
        git_status,
        os: std::env::consts::OS.to_string(),
        shell: std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
        available_tools,
    }
}

// --- Helper Functions ---

fn detect_project_type(dir: &PathBuf) -> Option<String> {
    if dir.join("Cargo.toml").exists() {
        Some("Rust".to_string())
    } else if dir.join("package.json").exists() {
        Some("Node.js".to_string())
    } else if dir.join("pyproject.toml").exists() || dir.join("requirements.txt").exists() {
        Some("Python".to_string())
    } else if dir.join("go.mod").exists() {
        Some("Go".to_string())
    } else {
        None
    }
}

fn get_git_info(dir: &PathBuf) -> (Option<String>, Option<String>) {
    // Check if this is even a git repo
    if !dir.join(".git").is_dir() {
        return (None, None);
    }

    // Get current branch name using git command
    let branch_output = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .current_dir(dir)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        });

    // Get a simplified status (clean or dirty)
    let status_output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(dir)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let has_changes = !output.stdout.is_empty();
                Some(if has_changes { "dirty" } else { "clean" }.to_string())
            } else {
                None
            }
        });

    (branch_output, status_output)
}

fn check_available_tools() -> AvailableTools {
    AvailableTools {
        git: which("git").is_ok(),
        node: which("node").is_ok(),
        npm: which("npm").is_ok(),
        cargo: which("cargo").is_ok(),
        python: which("python3").or_else(|_| which("python")).is_ok(),
        docker: which("docker").is_ok(),
    }
}