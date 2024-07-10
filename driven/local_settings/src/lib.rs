use std::env;

use domain::{
    constants::env::ENV_SETTING_DEFAULT_FOLDER_VAR,
    preinstalled::local_settings::{LocalSettings, LocalSettingsError, LocalSettingsProvider, PluginSettings},
};
use serde::{Deserialize, Serialize};
const SETTING_FOLDER_NAME: &str = ".lagrappe";
const PLUGINS_FOLDER_NAME: &str = "plugins";
const PLUGINS_DESCRIPTORS_FILE_NAME: &str = "plugins.json";

#[derive(Debug)]
pub struct LocalSettingsProviderImpl {
    pub user_settings_dir: String,
    pub plugins_dir: String,
    pub plugins_descriptors_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializablePluginSettings {
    pub name: String,
    pub is_enabled: bool,
    pub path_to_shared_lib: String,
}

impl From<SerializablePluginSettings> for PluginSettings {
    fn from(s: SerializablePluginSettings) -> Self {
        PluginSettings {
            name: s.name,
            is_enabled: s.is_enabled,
            path_to_shared_lib: s.path_to_shared_lib,
        }
    }
}

impl LocalSettingsProviderImpl {
    fn locate_user_settings_dir() -> Result<String, LocalSettingsError> {
        log::info!("Trying to locate user settings dir");
        env::var(ENV_SETTING_DEFAULT_FOLDER_VAR)
            .ok()
            .map(|v| {
                log::info!(
                    "Found env variable {}: {}",
                    ENV_SETTING_DEFAULT_FOLDER_VAR,
                    v
                );
                v
            })
            .or_else(|| {
                log::info!(
                    "Env variable {} not found, trying to get home dir",
                    ENV_SETTING_DEFAULT_FOLDER_VAR
                );
                dirs::home_dir()
                    .map(|path| {
                        let home_dir = path
                            .join(SETTING_FOLDER_NAME)
                            .to_str()
                            .map(|s| s.to_string());
                        log::info!("The detected home path is: {:?}", home_dir);
                        home_dir
                    })
                    .flatten()
            })
            .ok_or(LocalSettingsError::UnableToLocateSettingsDir)
    }

    //TODO: add description to the LocalSettingsError and map the error to it
    fn init_user_settings_if_not_exists(&self) -> Result<(), LocalSettingsError> {
        log::info!("Checking if user settings dir exists");
        if !std::path::Path::new(&self.user_settings_dir).exists() {
            log::info!("User settings dir does not exist, creating it");
            std::fs::create_dir(&self.user_settings_dir)
                .map_err(|_| LocalSettingsError::UnableToCreateSettingsDir)?;
        }

        log::info!("Checking if plugins dir exists");
        if !std::path::Path::new(&self.plugins_dir).exists() {
            log::info!("Plugins dir does not exist, creating it");
            std::fs::create_dir(&self.plugins_dir)
                .map_err(|_| LocalSettingsError::UnableToCreateSettingsDir)?;
        }

        log::info!("Checking if plugins descriptors file exists");
        if !std::path::Path::new(&self.plugins_descriptors_file).exists() {
            log::info!("Plugins descriptors file does not exist, creating it with empty array");
            let empty_plugins: Vec<SerializablePluginSettings> = Vec::new();
            let serialized = serde_json::to_string(&empty_plugins)
                .map_err(|_| LocalSettingsError::UnableToSerializeData)?;
            std::fs::write(&self.plugins_descriptors_file, serialized)
                .map_err(|_| LocalSettingsError::UnableToCreateSettingsDir)?;
        }

        Ok(())
    }
}

impl LocalSettingsProvider for LocalSettingsProviderImpl {
    fn new() -> Result<Self, LocalSettingsError>
    where
        Self: Sized,
    {
        let provider =
            LocalSettingsProviderImpl::locate_user_settings_dir().map(|user_settings_dir| {
                LocalSettingsProviderImpl {
                    plugins_dir: format!("{}/{}", user_settings_dir, PLUGINS_FOLDER_NAME),
                    plugins_descriptors_file: format!(
                        "{}/{}",
                        user_settings_dir, PLUGINS_DESCRIPTORS_FILE_NAME
                    ),
                    user_settings_dir,
                }
            })?;
        provider.init_user_settings_if_not_exists()?;
        Ok(provider)
    }

    fn get_all_data(&self) -> LocalSettings {
        todo!()
    }

    fn save(&self, data: String) -> Result<(), LocalSettingsError> {
        todo!()
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
