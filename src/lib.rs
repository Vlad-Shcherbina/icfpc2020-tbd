#![allow(unused_imports)]

use std::{path::{Path, PathBuf}};

pub mod png_files;
pub mod img_matrix;
pub mod operations;
pub mod parse_image;
pub mod tree;
pub mod squiggle;
pub mod webapi;
pub mod ufolang;
pub mod uforest;
pub mod tutorial;
pub mod ai_interface;
pub mod runners;
pub mod vec2;
pub mod bot_util;
pub mod local_bot_runner;

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
    // Can't simply return project_root().join(rel)
    // Need to deal with forward and backward slashes on Windows.
    let mut result = project_root();
    for part in rel.as_ref().iter() {
        result = result.join(part);
    }
    result
}

pub const API_KEY : &str = "ab93b0620ae245a8b92f4229eece9f3f";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_path() {
        assert!(project_path("Cargo.lock").exists());
        assert!(project_path("data/README.md").exists());
    }
}
