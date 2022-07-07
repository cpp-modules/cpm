use std::path::PathBuf;
use ignore::WalkBuilder;

pub fn publish(path: &PathBuf) {
    let mut walker = WalkBuilder::new(path);
    walker.add_custom_ignore_filename(".cpmignore");

    for result in walker.build() {
        let entry = result.unwrap();
        if entry.path().is_file() {
            println!("{}", entry.path().display());
        }
    }
}