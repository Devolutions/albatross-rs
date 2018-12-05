#[macro_use]
extern crate clap;

extern crate fs_extra;

use std::path::{PathBuf};

use std::env;
use std::path::Path;
use std::error::Error;

use std::process::{Command, Stdio};

use clap::App;

mod config;
use config::Config;

fn run_toolchain(_input: &str, config: &Config) {
    env::set_current_dir(config.tmp_dir.clone()).unwrap();

    let llvm_host_os = if cfg!(target_os = "linux") {
        "linux-gnu-ubuntu-16.04"
    } else if cfg!(target_os = "macos") {
        "apple-darwin"
    } else {
        "unknown"
    };

    let llvm_base_url = "https://releases.llvm.org";
    let llvm_version = "7.0.0";
    let llvm_host_arch = "x86_64";
    let llvm_basename = format!("clang+llvm-{}-{}-{}", llvm_version, llvm_host_arch, llvm_host_os);
    let llvm_filename = format!("{}.{}", llvm_basename, "tar.xz");

    let download_url = format!("{}/{}/{}",
        llvm_base_url, llvm_version, llvm_filename);

    if !PathBuf::from(llvm_filename.clone()).exists() {
        println!("downloading {}", llvm_filename);
        let _ = Command::new("curl")
            .arg("-O")
            .arg(download_url)
            .output()
            .expect("failed to download clang+llvm");
    }

    if !PathBuf::from(llvm_basename.clone()).exists() {
        println!("extracting {}", llvm_filename);
        let _ = Command::new("tar")
            .arg("-xf")
            .arg(llvm_filename)
            .output()
            .expect("failed to download clang+llvm");
    }

    let src_file = format!("{}", llvm_basename);
    let dst_file = format!("{}/llvm", config.toolchain_dir.display());

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    options.content_only = true;

    fs_extra::dir::create_all(dst_file.clone(),false).unwrap();
    fs_extra::dir::copy(src_file, dst_file, &options).unwrap();
}

fn run_sysroot(input: &str, config: &Config) {
    env::set_current_dir(Path::new(input)).unwrap();

    let cwd = env::current_dir().unwrap();
    let sysroot_name = cwd.file_name().unwrap().to_str().unwrap();

    let _ = Command::new("docker")
        .arg("build")
        .arg(".")
        .arg("-t")
        .arg(sysroot_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute docker");

    let docker_v = format!("{}/{}:/sysroot", config.sysroot_dir.display(), sysroot_name);
    let docker_c = "cd /sysroot && cp -R -L /lib lib && cp -R -L /usr usr";

    let _ = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(docker_v)
        .arg(sysroot_name)
        .arg("/bin/bash")
        .arg("-c")
        .arg(docker_c)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute docker");

    let src_file = "toolchain.cmake";
    let dst_file = format!("{}/{}.cmake", config.cmake_dir.display(), sysroot_name);

    let _ = Command::new("cp")
        .arg(src_file)
        .arg(dst_file)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to install cmake toolchain file");

}

fn run(matches: &clap::ArgMatches, config: &Config) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("toolchain", Some(_args)) => {
            run_toolchain("install", &config);
            Ok(())
        },
        ("sysroot", Some(args)) => {
            let input = args.value_of("input").unwrap();
            run_sysroot(input, &config);
            Ok(())
        },
        _ => { Err("unrecognized subcommand")? },
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let config = Config::load();
    let matches = app.version(crate_version!()).get_matches();

    match run(&matches, &config) {
        Ok(()) => { return }
        Err(e) => { println!("{}", e)}
    }
}
