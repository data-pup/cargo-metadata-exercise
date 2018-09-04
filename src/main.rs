extern crate cargo_metadata;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
// extern crate toml;

use std::path::{PathBuf, Path};

/// Given the path to the crate that we are buliding, return a path to the
/// lock file, by finding the workspace root.
fn _get_lockfile_path(path: Option<PathBuf>) -> PathBuf {
    // FIXUP: This is the line from the `cargo_metadata` documentation.
    let metadata = cargo_metadata::metadata(path.as_ref().map(Path::new)).unwrap();
    let workspace_root = metadata.workspace_root;
    let mut lockfile_path = PathBuf::from(workspace_root);
    lockfile_path.push("Cargo.lock");
    lockfile_path
}

fn main() {
    let lockfile_path = _get_lockfile_path(None);
    let output = format!("PATH: `{:?}`", lockfile_path);
    println!("{}", output);
}

