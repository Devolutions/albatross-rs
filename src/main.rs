#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate toml;

use std::env;
use std::path::Path;
use std::error::Error;

use std::process::Command;

use clap::App;

mod config;
use config::Config;

fn run_sysroot(input: &str) {
    env::set_current_dir(Path::new(input)).unwrap();

    let albatross_home = "/opt/albatross";

    let cwd = env::current_dir().unwrap();
    let sysroot_name = cwd.file_name().unwrap().to_str().unwrap();

    let docker_v = format!("{}/sysroot/{}:/sysroot", albatross_home, sysroot_name);
    let docker_c = "cd /sysroot && cp -R -L /lib lib && cp -R -L /usr usr";

    let _ = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(docker_v)
        .arg(sysroot_name)
        .arg("/bin/bash")
        .arg("-c")
        .arg(docker_c)
        .output()
        .expect("failed to execute docker");

    let src_file = "toolchain.cmake";
    let dst_file = format!("{}/cmake/{}.cmake", albatross_home, sysroot_name);

    let _ = Command::new("cp")
        .arg(src_file)
        .arg(dst_file)
        .output()
        .expect("failed to install cmake toolchain file");

}

fn run(matches: &clap::ArgMatches, _config: Option<Config>) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("sysroot", Some(args)) => {
            let input = args.value_of("input").unwrap();
            run_sysroot(input);
            Ok(())
        },
        _ => { Err("unrecognized subcommand")? },
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();

    let mut config : Option<Config> = None;

    if let Some(config_file) = matches.value_of("config") {
        match Config::new(Path::new(config_file)) {
            Ok(c) => { config = Some(c) },
            Err(e) => { println!("failed to read config file: {:?}", e) }
        }
    }

    match run(&matches, config) {
        Ok(()) => { return }
        Err(e) => { println!("{}", e)}
    }
}
