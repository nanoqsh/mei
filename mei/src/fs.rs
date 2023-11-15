use std::{fs, io::ErrorKind, path::Path};

pub fn create_dir<P>(dir: P)
where
    P: AsRef<Path>,
{
    fn create_dir_impl(dir: &Path) {
        if let Err(err) = fs::create_dir(dir) {
            assert!(
                err.kind() == ErrorKind::AlreadyExists,
                "failed to create {dir:?}: {err}",
            );
        }
    }

    create_dir_impl(dir.as_ref());
}

pub fn copy<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fn copy_impl(from: &Path, to: &Path) {
        if let Err(err) = fs::copy(from, to) {
            panic!("failed to copy from {from:?} to {to:?}: {err}");
        }
    }

    copy_impl(from.as_ref(), to.as_ref());
}

pub fn write<P, C>(path: P, contents: C)
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fn write_impl(path: &Path, contents: &[u8]) {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir(parent);
            }
        }

        if let Err(err) = fs::write(path, contents) {
            panic!("failed to write to {path:?}: {err}");
        }
    }

    write_impl(path.as_ref(), contents.as_ref());
}

pub fn read_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fn read_to_string_impl(path: &Path) -> String {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => panic!("failed to read from {path:?}: {err}"),
        }
    }

    read_to_string_impl(path.as_ref())
}
