fn main() {
    let assets_dir: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "assets"]
        .iter()
        .collect();
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // locate target directory from out directory
    let target_dir: Option<&std::path::Path> = {
        let mut cwd = Some(out_dir.as_path());
        loop {
            match cwd {
                // if path ends with "target", we assume this is correct dir
                Some(dir) if dir.ends_with("target") => break,
                // otherwise, keep going up in tree until we find "target" dir
                Some(dir) => cwd = dir.parent(),
                None => break,
            }
        }
        cwd
    };

    let target_assets_dir = std::path::PathBuf::from(target_dir.unwrap())
        .join(std::env::var("PROFILE").unwrap())
        .join("assets");

    // TODO: what I really should do is loop through all the files, check some hash to see if
    // they've changed, then update accordingly.removing everything every time something updates is
    // probably not the best, plus I think rust-analyzer is running this script.
    if target_assets_dir.exists() {
        std::fs::remove_dir_all(target_assets_dir.as_path()).unwrap();
    }

    // println!("cargo:warning={:?} -> {:?}", assets_dir, target_assets_dir);

    copy_dir::copy_dir(assets_dir, target_assets_dir).expect("Copy Failed 😢");
}
