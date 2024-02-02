use {
    crate::mei::Mei,
    std::{
        fs,
        io::ErrorKind,
        path::{Path, PathBuf},
    },
};

/// Creates a new, empty directory at the provided path.
///
/// This function calls [`create_dir`](std::fs::create_dir) internally
/// and therefore behaves the same, except for returning an error.
/// This function writes to the log and panics if it fails.
pub fn create_dir<P>(dir: P)
where
    P: AsRef<Path>,
{
    fn create_dir_impl(dir: &Path) {
        let dir = canonicalize(dir);
        if let Some(mei) = Mei::try_get() {
            if mei.verbose() {
                _ = mei.log().info(&format_args!(
                    "create directory at {dir}",
                    dir = dir.display(),
                ));
            }
        }

        if let Err(err) = fs::create_dir(&dir) {
            assert!(
                err.kind() == ErrorKind::AlreadyExists,
                "failed to create directory at {dir}: {err}",
                dir = dir.display(),
            );
        }
    }

    create_dir_impl(dir.as_ref());
}

/// Copies the contents of one file to another.
///
/// This function calls [`copy`](std::fs::copy) internally
/// and therefore behaves the same, except for returning an error.
/// This function writes to the log and panics if it fails.
pub fn copy<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fn copy_impl(from: &Path, to: &Path) {
        let from = canonicalize(from);
        let to = canonicalize(to);
        if let Some(mei) = Mei::try_get() {
            if mei.verbose() {
                _ = mei.log().info(&format_args!(
                    "copy from {from} to {to}",
                    from = from.display(),
                    to = to.display(),
                ));
            }
        }

        if let Err(err) = fs::copy(&from, &to) {
            panic!(
                "failed to copy from {from} to {to}: {err}",
                from = from.display(),
                to = to.display(),
            );
        }
    }

    copy_impl(from.as_ref(), to.as_ref());
}

/// Write a slice as the entire contents of a file.
///
/// This function calls [`write`](fn@std::fs::write) internally
/// and therefore behaves the same, except for returning an error.
/// It also automatically creates directories along the path if they don’t exist.
///
/// This function writes to the log and panics if it fails.
pub fn write<P, C>(path: P, contents: C)
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fn write_impl(path: &Path, contents: &[u8]) {
        let path = canonicalize(path);
        if let Some(mei) = Mei::try_get() {
            if mei.verbose() {
                _ = mei
                    .log()
                    .info(&format_args!("write to file {path}", path = path.display()));
            }
        }

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir(parent);
            }
        }

        if let Err(err) = fs::write(&path, contents) {
            panic!("failed to write to {path}: {err}", path = path.display());
        }
    }

    write_impl(path.as_ref(), contents.as_ref());
}

/// Read the entire contents of a file into a string.
///
/// This function calls [`read_to_string`](std::fs::read_to_string) internally
/// and therefore behaves the same, except for returning an error.
///
/// This function writes to the log and panics if it fails.
pub fn read_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fn read_to_string_impl(path: &Path) -> String {
        let path = canonicalize(path);
        if let Some(mei) = Mei::try_get() {
            if mei.verbose() {
                _ = mei
                    .log()
                    .info(&format_args!("read file {path}", path = path.display()));
            }
        }

        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => panic!("failed to read from {path}: {err}", path = path.display()),
        }
    }

    read_to_string_impl(path.as_ref())
}

fn canonicalize(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_owned())
}
