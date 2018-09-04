extern crate cargo_metadata;
#[macro_use]
extern crate serde_derive;
// extern crate serde_json;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

// FIXUP: Define lockfile structure.
#[derive(Debug, Deserialize)]
struct _CargoLockfile {
    package: Vec<_CargoLockfileDependency>,
}

/// FIXUP: This almost certainly needs some revision.
#[derive(Debug, Deserialize)]
struct _CargoLockfileDependency {
    name: String,
    version: String,
    source: Option<String>,
    dependencies: Option<Vec<String>>
}


/// Given the path to the crate that we are buliding, return a path to the
/// lock file, by finding the workspace root.
fn _get_lockfile_path(path: Option<PathBuf>) -> PathBuf {
    // FIXUP: Add error handling here.
    let metadata = cargo_metadata::metadata(path.as_ref().map(Path::new)).unwrap();
    let workspace_root = metadata.workspace_root;
    let mut lockfile_path = PathBuf::from(workspace_root);
    lockfile_path.push("Cargo.lock");
    lockfile_path
}

/// Read the `Cargo.lock` file found at the given location.
fn _read_cargo_lock(path: &Path) -> _CargoLockfile {
    if !path.is_file() {
        // FIXUP: Add error handling here.
        panic!("BORKED: Couldn't find the `Cargo.lock` file.");
    }
    let mut cargo_file = File::open(path).unwrap();
    let mut cargo_contents = String::new();
    cargo_file.read_to_string(&mut cargo_contents).unwrap();

    toml::from_str(&cargo_contents).unwrap()
}

fn main() {
    let lockfile_path = _get_lockfile_path(None);
    println!("{}", format!("PATH: `{:?}`", lockfile_path));
    let _lock_file = _read_cargo_lock(&lockfile_path);
    println!("{:?}", _lock_file);
}
