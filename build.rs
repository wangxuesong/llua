use std::env;
use std::process::Command;
use std::fs;

fn main() {
    let lua_files = vec!["func", "local_var", "sample", "table"];
    for file in lua_files {
        Command::new("luac")
            .args(&["-o"])
            .arg(&format!("{}.out", file))
            .arg(format!("{}.lua", file))
            .status()
            .unwrap();
    }
}
