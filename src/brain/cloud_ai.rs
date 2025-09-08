use crate::config::AIConfig;
use serde::{Deserialize, Serialize};
use anyhow::Result;

// Structures to match the OpenAI API JSON schema
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub async fn process_with_cloud_ai(
    command: &str,
    context: &str, // We'll pass a stringified version of the context
    config: &AIConfig,
) -> Result<String> {
    let api_key = config.openai_api_key.as_ref().ok_or_else(|| {
        anyhow::anyhow!("OpenAI API key not configured. Please set the 'openai_api_key' in your config.")
    })?;

    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    // Construct a prompt that understands our CLI context
    let prompt = format!(
        "You are Axiom, an AI-powered command-line interface. Your goal is to help the user complete tasks based on natural language.
        
        The user is currently in this environment:
        {}
        
        The user gave this command: '{}'
        
        Your response must be ONLY a single, valid, shell command that will achieve the user's goal. Do not write any explanation, only the command itself.
        ",
        context, command
    );

    let request = OpenAIRequest {
        model: config.openai_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant that only responds with a single Linux shell command. No explanations.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.1, // Low temperature for more deterministic commands
    };

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("API request failed: {}", error_text));
    }

    let response_json: OpenAIResponse = response.json().await?;
    let command = response_json.choices[0].message.content.trim().to_string();

    Ok(command)
}