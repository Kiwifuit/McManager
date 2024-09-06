use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use lazy_static::lazy_static;

lazy_static! {
    static ref GIT_PATH: PathBuf =
        PathBuf::from_str("../.git/").expect("expected path to be created");
}

fn main() -> anyhow::Result<()> {
    let head = GIT_PATH.join("head");

    let mut ref_file =
        File::open(&head).context(format!("while opening ref file: {}", head.display()))?;
    let mut ref_content = String::new();

    ref_file
        .read_to_string(&mut ref_content)
        .context(format!("while reading ref file: {}", head.display()))?;

    ref_content.truncate(ref_content.len() - 1);

    let head_ref = ref_content.strip_prefix("ref: ").unwrap();
    let git_head_commit = GIT_PATH.join(head_ref);
    let mut git_head = File::open(&git_head_commit).context(format!(
        "while opening git head file: {:?}",
        git_head_commit.display()
    ))?;
    let mut git_sha = String::new();

    git_head.read_to_string(&mut git_sha).context(format!(
        "while reading git head file: {:?}",
        git_head_commit.display()
    ))?;

    println!("cargo::rustc-env=GIT_SHA_LONG={}", git_sha);
    println!(
        "cargo::rustc-env=GIT_SHA_SHORT={}",
        git_sha.get(..8).unwrap()
    );

    Ok(())
}
