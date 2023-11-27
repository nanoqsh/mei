use {
    crate::mei::Mei,
    std::{
        fs,
        io::ErrorKind,
        path::{Path, PathBuf},
    },
};

pub fn create_dir<P>(dir: P)
where
    P: AsRef<Path>,
{
    fn create_dir_impl(dir: &Path) {
        let dir = canonicalize(dir);
        let mei = Mei::get();
        if mei.verbose() {
            _ = mei.log().info(&format_args!(
                "create directory at {dir}",
                dir = dir.display(),
            ));
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

pub fn copy<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fn copy_impl(from: &Path, to: &Path) {
        let from = canonicalize(from);
        let to = canonicalize(to);
        let mei = Mei::get();
        if mei.verbose() {
            _ = mei.log().info(&format_args!(
                "copy from {from} to {to}",
                from = from.display(),
                to = to.display(),
            ));
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

pub fn write<P, C>(path: P, contents: C)
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fn write_impl(path: &Path, contents: &[u8]) {
        let path = canonicalize(path);
        let mei = Mei::get();
        if mei.verbose() {
            _ = mei
                .log()
                .info(&format_args!("write to file {path}", path = path.display()));
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

pub fn read_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fn read_to_string_impl(path: &Path) -> String {
        let path = canonicalize(path);
        let mei = Mei::get();
        if mei.verbose() {
            _ = mei
                .log()
                .info(&format_args!("read file {path}", path = path.display()));
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
