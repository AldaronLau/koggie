extern crate pkg_config;
use std::process::Command;
use std::path::{Path, PathBuf};

fn main() {
    if pkg_config::find_library("opusfile").is_ok() { return }

    let dep_opus = std::env::var("DEP_OPUS_ROOT").ok().expect("where's opus-sys?");
    let dep_opus = Path::new(&dep_opus);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let static_lib_path = out_dir.join("lib/libopusfile.a");

    if std::fs::metadata(static_lib_path).is_err() {
        build(&out_dir, dep_opus);
    }

    inform_cargo(out_dir, dep_opus);
}

#[cfg(windows)]
fn build(out_dir: &Path, dep_opus: &Path) {
    panic!("Heyyy, you have access to windows machine! Fix this and request a pull!")
}

#[cfg(windows)]
fn inform_cargo(_out_dir: &Path, _dep_opus: &Path) {
    panic!("Heyyy, you have access to windows machine! Fix this and request a pull!")
}

#[cfg(unix)]
fn build(out_dir: &Path, dep_opus: &Path) {
    std::env::set_current_dir("libopusfile").unwrap_or_else(|e| panic!("{}", e));

    success_or_panic(Command::new("./configure")
        .env("PKG_CONFIG_PATH", dep_opus.join("lib/pkgconfig").to_str().unwrap())
        .args(&["--disable-shared", "--enable-static",
                "--disable-doc",
                "--disable-maintainer-mode", // I don't know what this switch does but it
                                             // fixes a problem involving time stamps and
                                             // autotools.
                "--with-pic",
                "--prefix", out_dir.to_str().unwrap()]));
    success_or_panic(&mut Command::new("make"));
    success_or_panic(&mut Command::new("make").arg("install"));

    std::env::set_current_dir("..").unwrap_or_else(|e| panic!("{}", e));
}

#[cfg(all(unix, not(target_os = "linux")))]
fn inform_cargo(out_dir: &Path, dep_opus: &Path) {
    let out_str = out_dir.to_str().unwrap();
    println!("cargo:rustc-flags=-L native={}/lib -l static=opusfile", out_str);
}

#[cfg(target_os = "linux")]
fn inform_cargo(out_dir: &Path, dep_opus: &Path) {
    let opusfile_pc = out_dir.join("lib/pkgconfig/opusfile.pc");
    if std::fs::metadata(&opusfile_pc).is_err() {
        panic!("opusfile.pc EI OLE")
    }
    let opusfile_pc = opusfile_pc.to_str().unwrap();
    prepend("PKG_CONFIG_PATH", dep_opus.join("lib/pkgconfig"));
    pkg_config::Config::new().statik(true).find(opusfile_pc).unwrap();
}

fn prepend(var: &str, val: PathBuf) {
    let prefix = std::env::var(var).unwrap_or(String::new());
    let mut v = vec![val];
    v.extend(std::env::split_paths(&prefix));
    std::env::set_var(var, &std::env::join_paths(v).unwrap());
}

fn success_or_panic(cmd: &mut Command) {
    match cmd.output() {
        Ok(output) => if !output.status.success() {
            panic!("command exited with failure\n=== Stdout ===\n{}\n=== Stderr ===\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr))
        },
        Err(e)     => panic!("{}", e),
    }
}
