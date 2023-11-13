use std::{fs, io::ErrorKind, path::Path};

pub fn create_dir<P>(dir: P)
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    if let Err(err) = fs::create_dir(dir) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("failed to create {dir:?}: {err}");
        }
    }
}

pub fn copy<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let from = from.as_ref();
    let to = to.as_ref();
    if let Err(err) = fs::copy(from, to) {
        panic!("failed to copy from {from:?} to {to:?}: {err}");
    }
}

pub fn write<P, C>(path: P, contents: C)
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            create_dir(parent);
        }
    }

    if let Err(err) = fs::write(path, contents) {
        panic!("failed to write to {path:?}: {err}");
    }
}

pub fn read_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => panic!("failed to read from {path:?}: {err}"),
    }
}
