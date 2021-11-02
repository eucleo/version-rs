# version-rs
Utility to help build a version string for a rust app including git and compiler info.

You can use it by adding this to your Cargo.toml:
```
[build-dependencies]
version-rs = { git = "https://github.com/eucleo/version-rs.git" }
```

And this use a build.rs like this:
```
use std::io::Result;

use version_rs::version;

fn main() -> Result<()> {
    println!(
        "cargo:rustc-env=VERSION_STRING={}",
        version(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_MANIFEST_DIR")
        )
    );
    Ok(())
}
```
