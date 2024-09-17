use std::{
    env, fs,
    io::{self, BufRead},
    process, sync, thread,
};

macro_rules! cmd {
    ( $cmd:literal $(,)? ) => {
        std::process::Command::new($cmd)
    };
    ( $cmd:literal, $( $arg:expr ),+ $(,)? ) => {{
        let mut cmd = std::process::Command::new($cmd);
        cmd.args([
                $({
                    let a: &std::ffi::OsStr = $arg.as_ref();
                    a
                },)+
            ]);
        cmd

    }};
}

macro_rules! check {
    ( $cmd:literal $(,)? ) => {
        check_command(cmd!($cmd))
    };
    ( $cmd:literal, $( $arg:expr ),+ $(,)? ) => {
        check_command(cmd!($cmd, $( $arg ),+ ))
    };
}

fn check_command(mut cmd: std::process::Command) -> Result<(), String> {
    let status = cmd
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .status()
        .map_err(|e| e.to_string())?;

    match status.code() {
        Some(code) if code > 0 => Err(format!("Failed with code: {code}")),
        _ => Ok(()),
    }
}

macro_rules! output {
    ( $cmd:literal $(,)? ) => {
        get_output(cmd!($cmd))
    };
    ( $cmd:literal, $( $arg:expr ),+ $(,)? ) => {
        get_output(cmd!($cmd, $( $arg ),+ ))
    };
}

fn get_output(mut cmd: std::process::Command) -> Result<String, String> {
    let output = cmd
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    let out = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let err = String::from_utf8_lossy(&output.stderr);
    if output.stderr.is_empty() {
        Ok(out)
    } else {
        Err(err.to_string())
    }
}

fn main() -> Result<(), String> {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "make" => {
                check!("mkdir", "-p", "wasm-module")?;
                check!(
                    "cargo",
                    "build",
                    "--release",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--package",
                    "basic",
                )?;
                check!(
                    "wasm-bindgen",
                    "--target",
                    "web",
                    "./target/wasm32-unknown-unknown/release/basic.wasm",
                    "--out-dir",
                    "wasm-dist/basic",
                )?;
            }
            _ => help(),
        }
    } else {
        help()
    };
    Ok(())
}

fn help() {
    println!(
        "available options are:
    
    `cargo xtask make`
"
    );
}
