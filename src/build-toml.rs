//! Builds the Cargo.toml file with an example for each file in src.
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let dir = fs::read_dir("src")?;

    let mut output = String::from(
        r#"[package]
name = "rustwrites"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#,
    );

    for file in dir {
        let file = file?.file_name();
        let name = file.to_str();

        if let Some(name) = name {
            let name = &name[0..name.len() - 3];
            output += &format!(
                r#"
[[example]]
name = "{name}"
path = "src/{name}.rs"
"#
            )[..];
        }
    }

    fs::write("Cargo.toml", output)?;

    Ok(())
}
