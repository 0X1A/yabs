// Copyright (c) 2015 - 2016, Alberto Corona <ac@albertocorona.com>
// All rights reserved. This file is part of yabs, distributed under the BSD
// 3-Clause license. For full terms please see the LICENSE file.

extern crate toml;

use error::{YabsError, YabsErrorKind};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

pub struct Job {
    process: Child,
    command: String,
}

impl Job {
    pub fn new(raw: (String, Child)) -> Job {
        Job {
            command: raw.0,
            process: raw.1,
        }
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn yield_self(&mut self) -> Result<(), YabsError> {
        let status = self.process.wait()?;
        if !status.success() {
            if let Some(ref mut stderr) = self.process.stderr {
                let mut buffer = String::new();
                stderr.read_to_string(&mut buffer)?;
                info!("{}", buffer);
            }
            bail!(YabsErrorKind::Command(self.command(), status.code().unwrap_or(1)));
        }
        Ok(())
    }
}

pub fn parse_toml_file<T: AsRef<Path> + Clone>(file: T) -> Result<String, YabsError> {
    let mut buff = String::new();
    let mut file = File::open(&file)?;
    file.read_to_string(&mut buff)?;
    Ok(buff)
}

pub fn get_assumed_filename() -> Option<String> {
    if let Ok(current_dir) = env::current_dir() {
        if let Some(file_stem) = current_dir.file_stem() {
            let mut file_name = file_stem.to_string_lossy().into_owned();
            file_name.push_str(".toml");
            return Some(file_name);
        }
    }
    None
}

pub fn get_assumed_filename_for_dir(dir: &PathBuf) -> Option<PathBuf> {
    if let Some(file_stem) = dir.file_stem() {
        return Some(PathBuf::from(file_stem.to_string_lossy().into_owned() + ".toml"));
    }
    None
}

pub fn run_cmd(cmd: &str) -> Result<(), YabsError> {
    let command = Command::new("sh").arg("-c").arg(&cmd).spawn()?.wait_with_output()?;
    println!("{}", &cmd);
    if !command.status.success() {
        print!("{}", String::from_utf8(command.stderr)?);
        bail!(YabsErrorKind::Command(cmd.to_owned(), command.status.code().unwrap_or(1)));
    }
    print!("{}", String::from_utf8(command.stdout)?);
    Ok(())
}

pub fn spawn_cmd(cmd: &str) -> Result<Child, YabsError> {
    Ok(Command::new("sh").arg("-c").arg(&cmd).spawn()?)
}

pub trait PrependEach<T> {
    fn prepend_each(&self, pre: &str) -> Vec<String>;
}

// self.include.prepend_each("-I");
impl PrependEach<String> for Vec<String> {
    fn prepend_each(&self, pre: &str) -> Vec<String> {
        let mut clone = self.clone();
        for each in &mut clone {
            *each = pre.to_owned() + each;
        }
        clone
    }
}
