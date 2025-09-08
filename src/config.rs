// ... (existing code)

#[derive(Debug, Serialize, Deserialize)]
pub struct AIConfig {
    pub local_model: String,
    pub cloud_fallback: bool,
    #[serde(skip_serializing_if = "Option::is_none")] // Don't print the key to logs!
    pub openai_api_key: Option<String>, // Make this an Option so it can be absent
    pub openai_model: String, // e.g., "gpt-3.5-turbo"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub rich_output: bool,
}

impl Default for AxiomConfig {
    fn default() -> Self {
        Self {
            ai: AIConfig {
                local_model: "llama2".to_string(),
                cloud_fallback: false,
                openai_api_key: None, // Default to no key
                openai_model: "gpt-3.5-turbo".to_string(),
            },
            ui: UIConfig {
                theme: "dark".to_string(),
                rich_output: true,
            },
        }
    }
}

// ... (existing code)