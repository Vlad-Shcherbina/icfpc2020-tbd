use std::path::{Path, PathBuf};

pub mod png_files;
pub mod img_matrix;
pub mod operations;
pub mod parse_image;


pub fn hello() {
    println!("hello");
}

fn project_root() -> PathBuf {
    let exe = std::fs::canonicalize(std::env::args().next().unwrap()).unwrap();
    let mut path: &Path = &exe;
    while path.file_name().unwrap() != "icfpc2020-tbd" {
        path = path.parent().unwrap();
    }
    path.to_owned()
}

pub fn project_path(rel: impl AsRef<Path>) -> PathBuf {
    project_root().join(rel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_path() {
        assert!(project_path("Cargo.lock").exists());
    }
}