use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use tempfile::{tempdir, TempDir};

fn setup_source() -> Result<TempDir> {
    let dir = tempdir()?;
    Command::new("cargo")
        .args(["new", "--lib", "primary"])
        .current_dir(&dir)
        .output()?;
    Ok(dir)
}

fn add_dep(dir: &Path, dep: &str) -> Result<()> {
    Command::new("cargo")
        .args(["new", "--lib"])
        .arg(&dep)
        .current_dir(&dir)
        .output()?;
    let primary_path = dir.join("primary");
    Command::new("cargo")
        .args(["add", "--path"])
        .arg(format!("../{}", dep))
        .current_dir(&primary_path)
        .output()?;
    Ok(())
}

#[test]
fn cleanup_3_removed_libs() -> Result<()> {
    let testdir = setup_source()?;
    let primary_toml_path = testdir.path().join("primary/Cargo.toml");
    let original_cargo_toml = {
        let mut string = String::new();
        File::open(primary_toml_path)?.read_to_string(&mut string)?;
        string
    };

    add_dep(testdir.path(), "dep0")?;
    add_dep(testdir.path(), "dep1")?;
    add_dep(testdir.path(), "dep2")?;

    Ok(())
}
