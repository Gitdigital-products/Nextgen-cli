use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub commands: HashMap<String, PluginCommand>,
}

#[derive(Debug, Deserialize)]
pub struct PluginCommand {
    pub description: String,
    pub pattern: String,
    pub action: String,
}

pub fn load_plugins() -> anyhow::Result<Vec<PluginManifest>> {
    // TODO: Implement plugin discovery and loading
    Ok(Vec::new())
}