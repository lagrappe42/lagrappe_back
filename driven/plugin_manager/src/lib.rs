use std::{collections::HashMap, ffi::OsStr, sync::Arc};

use domain::plugin::{AppExtendManager, Plugin, PLUGIN_ENTRYPOINT_SYMBOL};
use libloading::{Library, Symbol};

pub struct AppExtendManagerImpl {
    extends: HashMap<String, Arc<Box<dyn Plugin>>>,
    loaded_libraries: Vec<Library>,
}

impl AppExtendManager for AppExtendManagerImpl {
    fn new() -> AppExtendManagerImpl {
        AppExtendManagerImpl {
            extends: HashMap::new(),
            loaded_libraries: Vec::new(),
        }
    }

    unsafe fn load_extend(&mut self, filename: &OsStr) {
        type ExtendCreator = unsafe fn() -> *mut dyn Plugin;

        match Library::new(filename) {
            Ok(lib) => {
                self.loaded_libraries.push(lib);

                let lib = self.loaded_libraries.last().unwrap();
                match lib.get::<Symbol<ExtendCreator>>(PLUGIN_ENTRYPOINT_SYMBOL) {
                    Ok(constructor) => {
                        let boxed_raw = constructor();
                        if boxed_raw.is_null() {
                            eprintln!("Constructor returned null pointer.");
                            return;
                        }

                        let extend = Box::from_raw(boxed_raw);

                        println!("Extend {} loaded.", extend.name());

                        self.extends
                            .insert(extend.name().to_string(), Arc::new(extend));
                    }
                    Err(e) => {
                        eprintln!("Failed to load symbol: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to load library: {}", e);
            }
        }
    }
}
/*

    /// Select the extension with the specified name
    pub fn select<T: Into<String>>(&self, target: T) -> UcenterResult<Arc<Box<UcenterApp>>> {
        let key: String = target.into();

        self.extends
            .get(&key)
            .map(|v| {
                // v is actually an Arc instance; cloning here incurs no significant performance overhead
                // and allows ownership (read-only). Arc is an atomic reference counting smart pointer,
                // so it can be safely used across threads.
                v.clone()
            })
            .ok_or(UcenterError::system_subsystem_error(None))
    }
}
    */
