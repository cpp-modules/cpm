use std::{path::PathBuf, process::Command};

pub struct Build {
    compiler_type: CompilerType,
    out: String,
    options: Vec<OptionType>,
    inputs: Vec<String>,
}

pub enum CompilerType {
    GCC,
    CLANG,
    MSVC,
}

pub enum OptionType {
    ImplicitDir(String, String),
    Flag(String),
    FlagWithArgument(String, String),
}

impl Build {
    pub fn new() -> Build {
        Build {
            compiler_type: CompilerType::GCC,
            out: String::from(""),
            options: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub fn outdir(&mut self, dir: PathBuf) -> &mut Build {
        self.out = dir.to_str().unwrap().to_string();
        self
    }

    pub fn libdir(&mut self, dir: PathBuf) -> &mut Build {
        self.options.push(OptionType::ImplicitDir(
            "-L".to_string(),
            dir.to_str().unwrap().to_string(),
        ));
        self
    }

    pub fn source(&mut self, path: PathBuf) -> &mut Build {
        self.inputs.push(path.to_str().unwrap().to_string());
        self
    }

    pub fn linklib(&mut self, name: &str) -> &mut Build {
        self.options
            .push(OptionType::ImplicitDir("-l".to_string(), name.to_string()));
        self
    }

    pub fn flag(&mut self, flag: &str) -> &mut Build {
        self.options.push(OptionType::Flag(flag.to_string()));
        self
    }

    pub fn exec(&self) {
        let mut cmd = Command::new("g++");
        cmd.args(&self.inputs);
        cmd.args(["-o", &self.out]);

        for option in &self.options {
            match option {
                OptionType::ImplicitDir(flag, dir) => {
                    cmd.arg(flag.to_owned() + dir);
                }
                OptionType::Flag(flag) => {
                    cmd.arg(&flag);
                }
                OptionType::FlagWithArgument(flag, arg) => {
                    cmd.arg(flag.to_owned() + "=" + arg);
                }
            }
        }

        cmd.spawn();
    }
}
