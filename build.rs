use std::process::Command;

fn main() {
    let lua_files = vec!["func", "local_var", "sample", "table", "upvalue"];
    for file in lua_files {
        Command::new("luac")
            .args(&["-o"])
            .arg(&format!("{}.out", file))
            .arg(format!("{}.lua", file))
            .status()
            .unwrap();
    }
    let test_lua_files = vec!["func", "sample", "global", "print"];
    for file in test_lua_files {
        Command::new("luac")
            .args(&["-o"])
            .arg(&format!("tests/{}.out", file))
            .arg(format!("tests/{}.lua", file))
            .status()
            .unwrap();
    }
}
