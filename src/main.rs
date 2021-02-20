extern crate notify;

use std::{ffi::OsStr, path::{self, Path, PathBuf}};
use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};
use std::{result, sync::mpsc::channel};
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    let path = Path::new("C:/Drops");
    watcher.watch(path, RecursiveMode::Recursive);


    loop {
        match rx.recv() {
            Ok(event) => handle_event(event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn handle_event(event: DebouncedEvent) {
    match event {
        DebouncedEvent::Create(path)=> handle_create(path),
        _ => ()
    }
}

fn handle_create(path_buf: PathBuf) {
    println!("Created event! {:?}", path_buf);

    if path_buf.is_file() {
        println!("This is a file {:?}", path_buf.file_stem());
        
        let new_path = get_path(path_buf.file_stem());

    }
}

fn get_path(maybe_file: Option<&OsStr>) -> Option<&str> {
    let file_name = maybe_file?.to_str()?;

    return Some(file_name.replace("-", "/").as_str());
}