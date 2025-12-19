use std::{collections::HashMap, fs::File, io::Read};


fn build_env_map<'a>(file_content: &'a str) -> HashMap<&'a str, &'a str> {
    let mut map  = HashMap::new();
    for line in file_content.lines() {
        // Ignore full line comments
        if line.trim_start().starts_with("#") { continue };
        // Ignore empty lines.
        if line.trim().is_empty() { continue };

        // Split line on =
        let (name, value) = match line.split("=").take(2).collect::<Vec<_>>()[..] {
            [a, b, ..] => (a, b),
            _ => unreachable!()
        };
        
        // Trim the name "  NAME    = .." => "NAME"
        let name = name.trim();

        // Trim the value and remove comments
        let value = value.split("#").next().unwrap().trim();

        if map.contains_key(name) { panic!("Found a duplicate definition for '{}' in '.env'. Values: (1)={}, (2)={}", name, map.get(name).unwrap(), value) };
        map.insert(name,value);
    }

    map
}

// Building production
// #[cfg(not(debug_assertions))]
fn main() {
    // Only rerun if .env changes.
    println!("cargo::rerun-if-changed=.env");
    
    let Ok(mut file) = File::open(".env") else { panic!("Build failed: Could not open/find .env file."); };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Build failed: Could not read .env file.");    

    let map = build_env_map(&s);
    
    let mut generated = String::new();
    generated += r#"pub fn acquire_string(var: &EnvVariable) -> &'static str {use EnvVariable::*;match var {"#;

    for (key, value) in map {
        generated += &format!("\n{key} => \"{value}\",");
    }

    generated += "}}";

    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(
        format!("{}/generated.rs", out_dir),
        generated,
    ).unwrap();
}

// #[cfg(debug_assertions)]
// fn main() {}

// fn main() {
//     // 1. Read or compute values you need at build time
//     //    e.g. read a .env file, query git, generate code, etc.

//     // 2. Communicate with Cargo using println! directives
//     //    These lines tell Cargo how to adjust the build.

//     // Example: set an environment variable for rustc
//     println!("cargo:rustc-env=API_KEY=supersecret");

//     // Example: rerun build script if .env changes
//     println!("cargo:rerun-if-changed=.env");

//     // Example: write out a generated file into OUT_DIR
//     let out_dir = std::env::var("OUT_DIR").unwrap();
//     std::fs::write(
//         format!("{}/generated.rs", out_dir),
//         r#"pub const API_KEY: &str = "supersecret";"#,
//     ).unwrap();
// }