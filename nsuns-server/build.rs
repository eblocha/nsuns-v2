use std::process;

fn git_revision_hash() -> Option<String> {
    let result = process::Command::new("git")
        .args(&["rev-parse", "--short=10", "HEAD"])
        .output();

    result.ok().and_then(|output| {
        let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}

fn main() {
    if let Some(rev) = git_revision_hash() {
        println!("cargo:rustc-env=NSUNS_BUILD_GIT_HASH={}", rev);
    }
}
