
use std::env;
use std::path::{PathBuf};

use fs_extra::dir::*;

#[derive(Clone)]
pub struct Config {
    pub albatross_home: PathBuf,
    pub toolchain_dir: PathBuf,
    pub sysroot_dir: PathBuf,
    pub cmake_dir: PathBuf,
    pub tmp_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Config {
            albatross_home: PathBuf::new(),
            toolchain_dir: PathBuf::new(),
            sysroot_dir: PathBuf::new(),
            cmake_dir: PathBuf::new(),
            tmp_dir: PathBuf::new(),
        }
    }

    pub fn load_default(&mut self) {

        let home_env = if cfg!(target_os = "windows") {
            "USERPROFILE"
        } else {
            "HOME"
        };

        let home_dir = env::var(home_env).unwrap();

        self.albatross_home = PathBuf::from(home_dir).join(".albatross");
    }

    pub fn load_env(&mut self) {
        if let Ok(val) = env::var("ALBATROSS_HOME") {
            self.albatross_home = PathBuf::from(Some(val).unwrap());
        }
    }

    pub fn setup(&mut self) {
        self.toolchain_dir = self.albatross_home.clone().join("toolchain");
        self.sysroot_dir = self.albatross_home.clone().join("sysroot");
        self.cmake_dir = self.albatross_home.clone().join("cmake");
        self.tmp_dir = self.albatross_home.clone().join("tmp");

        create_all(&self.albatross_home, false).unwrap();
        create_all(&self.toolchain_dir, false).unwrap();
        create_all(&self.sysroot_dir, false).unwrap();
        create_all(&self.cmake_dir, false).unwrap();
        create_all(&self.tmp_dir, false).unwrap();
    }

    pub fn load() -> Self {
        let mut config = Config::new();
        config.load_default();
        config.load_env();
        config.setup();
        config
    }
}
