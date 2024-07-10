use datasource::DataSourceImpl;
use domain::{
    plugin::{AppExtendManager, DataSource},
    preinstalled::local_settings::LocalSettingsProvider,
    preinstalled::{
        async_trait::async_trait,
        plugin_hub::{Plugin, PluginHubError, PluginsHubService},
    },
};
use local_settings::LocalSettingsProviderImpl;
use plugin_hub::PluginHubServiceImpl;
use std::{env, ffi::OsStr, path::PathBuf, process::exit, thread};

fn current_dir() -> String {
    let mut current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    current_dir.push_str("/driven");
    log::info!("Current directory: {}", current_dir);
    current_dir
}

//hello
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting the application");

    let mut manager = plugin_manager::AppExtendManagerImpl::new();

    unsafe {
        manager.load_extend(OsStr::new("/home/amyroshn/Documents/extendmepls_workspace/rust_plugin_example/target/release/librust_plugin_example.so").as_ref());
    }

    let local_settings_provider = LocalSettingsProviderImpl::new().unwrap();
    println!("local settings provider: {:?}", local_settings_provider);

    let plugin_hub_service_impl = PluginHubServiceImpl::new();
    println!("plugin hub service impl: {:?}", plugin_hub_service_impl);
    let _ = plugin_hub_service_impl
        .install(
            local_settings_provider.plugins_dir.as_str(),
            plugin_hub_service_impl.get_all().await.get(0).unwrap(),
        )
        .await;

    /*

    let data_source = DataSourceImpl::new(current_dir());
    data_source
        .watch(|| println!("\nCallback is triggered\n"))
        .unwrap();

    let files_data = data_source.get_all_data().unwrap();
    if files_data.is_empty() {
        println!("No files found");
        exit(1)
    }
    println!("Files found: {}", files_data.len());
    files_data.into_iter().for_each(|(k, v)| {
        //println!("{}:\n{:?}\n\n\n", k, String::from_utf8_lossy(&v));
    });

    thread::park()
    */
}
