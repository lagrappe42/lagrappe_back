use std::{error::Error, fs, path::Path};

use domain::preinstalled::{
    async_trait::async_trait,
    plugin_hub::{Plugin, PluginHubError, PluginsHubService},
};
use git2::Repository;
use serde::Deserialize;

//TODO: provide real implementation, when the plugin structure will be agreed
#[derive(Debug)]
pub struct PluginHubServiceImpl {
    plugins: Vec<Plugin>,
}

impl PluginHubServiceImpl {
    pub fn new() -> Self {
        log::info!("Creating mock PluginHubServiceImpl");
        PluginHubServiceImpl {
            plugins: vec![Plugin {
                name: "rust_plugin_example ".to_string(),
                github_url: "https://github.com/lagrappe42/rust_plugin_example".to_string(),
                description: "my awesome plugin".to_string(),
            }],
        }
    }
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
    lib: Option<toml::Value>,
    dependencies: Option<toml::Value>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[async_trait]
impl PluginsHubService for PluginHubServiceImpl {
    async fn get_all(&self) -> &Vec<Plugin> {
        log::info!("Mock: Getting all plugins from plugin hub");
        &self.plugins
    }
    async fn install(&self, plugins_dir: &str, plugin: &Plugin) -> Result<(), PluginHubError> {
        log::info!(
            "Trying to install plugin {} to {}",
            plugin.name,
            plugins_dir
        );
        let full_path = format!("{}/{}", plugins_dir, plugin.name);
        GitService::clone_repo(&plugin.github_url, &full_path)
            .map_err(|_| PluginHubError::InstallationError)?;
        let cargo_toml_content =
            fs::read_to_string(format!("{}/Cargo.toml", full_path)).map_err(|e| {
                log::error!("Failed to read Cargo.toml: {}", e);
                if let Err(e) = GitService::delete_repo(&full_path) {
                    log::error!("Failed to delete repo: {}", e);
                }
                PluginHubError::InstallationError
            })?;
        let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content).map_err(|e| {
            log::error!("Failed to parse Cargo.toml: {}", e);
            if let Err(e) = GitService::delete_repo(&full_path) {
                log::error!("Failed to delete repo: {}", e);
            }
            PluginHubError::InstallationError
        })?;
        log::info!("Parsed Cargo.toml: {:?}", cargo_toml);
        Ok(())
    }
}

pub struct GitService;

impl GitService {
    pub fn clone_repo(repo_url: &str, dest_path: &str) -> Result<Repository, Box<dyn Error>> {
        log::info!("Cloning repo from {} to {}", repo_url, dest_path);
        let repo = Repository::clone(repo_url, Path::new(dest_path));
        if repo.is_err() {
            log::error!("Failed to clone repo");
        } else {
            log::info!("Repo cloned successfully");
        }
        Ok(repo?)
    }

    pub fn delete_repo(repo_path: &str) -> Result<(), Box<dyn Error>> {
        log::info!("Deleting repo at {}", repo_path);
        std::fs::remove_dir_all(repo_path)?;
        Ok(())
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
