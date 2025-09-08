use crate::config::AIConfig;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

// Structures to match the Ollama API JSON schema
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn process_with_local_ai(
    command: &str,
    context: &str,
    config: &AIConfig,
) -> Result<String> {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate"; // Default Ollama endpoint

    // Construct a detailed prompt for the local model
    let prompt = format!(
        "You are Axiom, an AI-powered command-line interface. Your goal is to help the user complete tasks based on natural language.
        
        The user is currently in this environment:
        {}
        
        The user gave this command: '{}'
        
        Your response must be ONLY a single, valid, shell command that will achieve the user's goal. Do not write any explanation, only the command itself.
        ",
        context, command
    );

    let request = OllamaRequest {
        model: config.local_model.clone(),
        prompt: prompt,
        stream: false,
    };

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?;

    // Check if Ollama is running and the model is available
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Ollama request failed: {}. Is Ollama installed and running? Try 'ollama serve'", error_text));
    }

    let response_json: OllamaResponse = response.json().await?;
    let ai_output = response_json.response.trim().to_string();

    // Extract just the command from the response (models sometimes add explanation)
    let command_line = extract_command_from_output(&ai_output);

    Ok(command_line)
}

// Helper function to extract the first shell command from the AI's response
fn extract_command_from_output(output: &str) -> String {
    // Look for lines that seem like commands (start with $, or are in code blocks)
    for line in output.lines() {
        let trimmed = line.trim();
        // Remove leading $ or > if present (common in model responses)
        let cleaned_line = trimmed
            .strip_prefix('$')
            .or_else(|| trimmed.strip_prefix('>'))
            .unwrap_or(trimmed)
            .trim();

        // If the line looks like a command (not empty, not just punctuation)
        if !cleaned_line.is_empty() && !cleaned_line.chars().all(|c| c.is_ascii_punctuation()) {
            return cleaned_line.to_string();
        }
    }
    
    // Fallback: return the entire output trimmed
    output.trim().to_string()
}