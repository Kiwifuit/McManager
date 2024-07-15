use std::process::Command;

const GIT_SHA_MIN: usize = 8;

fn main() {
    println!("cargo::rerun-if-changed=.git/HEAD");
    let git_sha_long = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("should be able to get commit hash")
            .stdout,
    )
    .expect("commit hash should be valid utf8")
    .trim()
    .to_string();
    let git_sha_short = &git_sha_long[..GIT_SHA_MIN];

    println!("cargo:rustc-env=GIT_SHA_LONG={}", &git_sha_long);
    println!("cargo:rustc-env=GIT_SHA_SHORT={}", &git_sha_short);
}
