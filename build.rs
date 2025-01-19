use std::process::Command;
use {
    std::{
        env,
        io,
    },
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("./assets/ico.ico")
            .compile()?;
    }

    let output = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .expect("Failed to execute git");

    let git_tag = String::from_utf8(output.stdout).expect("Invalid UTF-8 in git output");
    let git_tag = git_tag.trim();

    println!("cargo:rustc-env=GIT_TAG={}", git_tag);
    
    Ok(())
}