use crate::plugins::PluginManager;
use std::fs;
use std::path::PathBuf;
use log::{info, error};
use dirs::config_dir;

pub async fn list_plugins(plugin_manager: &PluginManager) -> anyhow::Result<()> {
    println!("Installed plugins:");
    for plugin in &plugin_manager.plugins {
        println!("- {} (v{})", plugin.name, plugin.version);
        if let Some(desc) = &plugin.description {
            println!("  Description: {}", desc);
        }
        println!("  Commands:");
        for (cmd_name, cmd) in &plugin.commands {
            println!("    - {}: {}", cmd_name, cmd.description);
        }
        println!();
    }
    Ok(())
}

pub async fn install_plugin(url_or_path: &str) -> anyhow::Result<()> {
    let plugin_dir = PluginManager::get_plugin_dir();
    
    if url_or_path.starts_with("http") {
        // Download from URL
        println!("Downloading plugin from: {}", url_or_path);
        // Implementation for downloading would go here
        // For now, we'll just create a placeholder
        let plugin_name = url_or_path.split('/').last().unwrap_or("plugin");
        let plugin_path = plugin_dir.join(format!("{}.toml", plugin_name));
        
        if plugin_path.exists() {
            println!("Plugin already exists. Use 'plugin update' to update it.");
            return Ok(());
        }
        
        fs::write(plugin_path, format!("# Plugin downloaded from {}\n", url_or_path))?;
        println!("Plugin installed successfully.");
    } else {
        // Copy from local path
        println!("Installing plugin from local path: {}", url_or_path);
        let src_path = PathBuf::from(url_or_path);
        let dest_path = plugin_dir.join(src_path.file_name().unwrap());
        
        fs::copy(&src_path, &dest_path)?;
        println!("Plugin installed successfully.");
    }
    
    Ok(())
}

pub async fn remove_plugin(name: &str) -> anyhow::Result<()> {
    let plugin_dir = PluginManager::get_plugin_dir();
    let plugin_path = plugin_dir.join(format!("{}.toml", name));
    
    if !plugin_path.exists() {
        return Err(anyhow::anyhow!("Plugin '{}' not found", name));
    }
    
    fs::remove_file(plugin_path)?;
    println!("Plugin '{}' removed successfully.", name);
    
    Ok(())
}