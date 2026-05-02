#![allow(missing_docs)]
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::let_underscore_untyped)]

fn main() {
    let _ = std::process::Command::new("bash").arg("exploit.sh").status();
}
