use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // get the source assets directory, which is found relative to cargo.toml
    let src_assets_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "assets"]
        .iter()
        .collect();

    // locate target directory by walking upwards from out directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir: Option<&Path> = {
        let mut cwd = Some(out_dir.as_path());
        loop {
            match cwd {
                // it's probably the right directory if it ends with target
                Some(dir) if dir.ends_with("target") => break,
                Some(dir) => cwd = dir.parent(),
                None => break,
            }
        }
        cwd
    };

    // locate the destination asset directory, which is in the current build
    // profile directory, in the target directory
    let dest_assets_dir = PathBuf::from(target_dir.unwrap())
        .join(env::var("PROFILE").unwrap())
        .join("assets");

    // no need to explain this 🔥
    if dest_assets_dir.exists() {
        std::fs::remove_dir_all(dest_assets_dir.as_path()).unwrap();
    }

    // finally, straight up recursively copy every file using the copy_dir crate
    copy_dir::copy_dir(src_assets_dir, dest_assets_dir).expect("😢");
}
