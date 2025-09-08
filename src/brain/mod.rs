use crate::config::AxiomConfig;
use crate::context::EnvironmentContext;
use crate::core::executor;

pub async fn process_command(
    command_str: &str,
    context: &EnvironmentContext,
    _config: &AxiomConfig, // We'll use this later for AI config
) -> anyhow::Result<()> {
    println!("Axiom is thinking... 🤔");

    // Convert the command to lowercase for easier matching
    let command = command_str.to_lowercase();

    // --- Pattern Matching Logic ---
    // This is our simple, rule-based "brain".
    // We will match on phrases in the natural language command.

    if command.contains("find file") || command.contains("look for file") {
        println!("I'll help you find a file.");
        // We can make this more sophisticated later
        let find_output = executor::execute_shell_command("find", &[".", "-name", "*.rs"], context)?;
        println!("Found Rust files:\n{}", find_output);

    } else if command.contains("list file") || command.contains("ls") || command.contains("directory") {
        println!("Listing files for you:");
        // Use the system's `ls` or our own logic later
        let ls_output = executor::execute_shell_command("ls", &["-la"], context)?;
        println!("{}", ls_output);

    } else if command.contains("what's in") || command.contains("show me") {
        // A more general "show me things" command
        println!("Let me show you what's here:");
        let ls_output = executor::execute_shell_command("ls", &["-la"], context)?;
        println!("{}", ls_output);

    } else if command.contains("git status") || command.contains("git status") {
        // Leverage the context we already gathered!
        if let Some(status) = &context.git_status {
            println!("Git status is: {}", status);
        } else {
            println!("This doesn't seem to be a git repository.");
        }

    } else {
        // The fallback if we don't understand the command: just print the context.
        println!("I'm still learning. I heard you say: '{}'", command_str);
        println!("Let me show you what I *do* know about your environment:");
        println!("You are in: {}", context.current_dir_str);
        println!("OS: {}", context.os);
        println!("Project type: {:?}", context.project_type);
        println!("Git branch: {:?}", context.git_branch);
    }

    Ok(())
}