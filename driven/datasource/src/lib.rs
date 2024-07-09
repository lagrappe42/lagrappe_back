use domain::plugin::{DataSource, OnChangedCallback};
use notify::{poll::ScanEvent, Config, PollWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    thread,
};

pub struct DataSourceImpl {
    path: PathBuf,
}

impl DataSourceImpl {
    pub fn new(path: String) -> Self {
        DataSourceImpl {
            path: PathBuf::from(path),
        }
    }

    //TODO: make async
    fn store_files_in_memory(&self, root: &str) -> HashMap<String, Vec<u8>> {
        let mut file_map: HashMap<String, Vec<u8>> = HashMap::new();
        let root_path = Path::new(root);

        if root_path.is_dir() {
            for entry in fs::read_dir(root_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() {
                    let file_content = fs::read(&path).unwrap();
                    file_map.insert(path.to_str().unwrap().to_string(), file_content);
                } else if path.is_dir() {
                    let sub_map = self.store_files_in_memory(path.to_str().unwrap());
                    file_map.extend(sub_map);
                }
            }
        }
        file_map
    }
}

impl DataSource for DataSourceImpl {
    fn watch(&self, on_changed: OnChangedCallback) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = std::sync::mpsc::channel();
        let path_clone = self.path.to_path_buf();
        enum Message {
            Event(notify::Result<notify::Event>),
            Scan(ScanEvent),
        }

        let tx_c = tx.clone();
        let mut watcher = PollWatcher::with_initial_scan(
            move |watch_event| {
                tx_c.send(Message::Event(watch_event)).unwrap();
            },
            Config::default().with_poll_interval(std::time::Duration::from_secs(2)),
            move |scan_event| {
                tx.send(Message::Scan(scan_event)).unwrap();
            },
        )?;

        thread::spawn(move || {
            watcher
                .watch(path_clone.as_ref(), RecursiveMode::Recursive)
                //TODO: handle error
                .unwrap();
            for res in rx {
                match res {
                    Message::Event(e) => println!("Watch event {e:?}"),
                    Message::Scan(e) => println!("Scan event {e:?}"),
                }
                on_changed();
            }
        });
        Ok(())
    }

    fn get_all_data(&self) -> Result<HashMap<String, Vec<u8>>, Box<dyn Error>> {
        //TODO: handle error
        Ok(self.store_files_in_memory(self.path.to_str().unwrap()))
    }
}
