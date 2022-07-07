use super::module_toml::Module;
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

    pub fn output(&mut self, dir: &PathBuf) -> &mut Build {
        self.out = dir.to_str().unwrap().to_string();
        self
    }

    pub fn libdir(&mut self, dir: &PathBuf) -> &mut Build {
        self.options.push(OptionType::ImplicitDir(
            "-L".to_string(),
            dir.to_str().unwrap().to_string(),
        ));
        self
    }

    pub fn includedir(&mut self, dir: &PathBuf) -> &mut Build {
        self.options.push(OptionType::ImplicitDir(
            "-I".to_string(),
            dir.to_str().unwrap().to_string(),
        ));
        self
    }

    pub fn source(&mut self, path: &PathBuf) -> &mut Build {
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

    pub fn flag_with_argument(&mut self, flag: &str, argument: &str) -> &mut Build {
        self.options.push(OptionType::FlagWithArgument(
            flag.to_string(),
            argument.to_string(),
        ));
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

        cmd.spawn().unwrap();
    }
}

pub fn build_root(root_path: &PathBuf) {
    let mut builder: Build = Build::new();

    let module_toml =
        std::fs::read_to_string(root_path.join("module.toml")).expect("Failed to read module.toml");
    let module_toml: Module = toml::from_str(&module_toml).expect("Failed to parse module.toml");

    for (dependency, _) in &module_toml.dependencies {
        build_submodule(root_path, &dependency);
    }

    for filename in &module_toml.sources.source {
        builder.source(&root_path.join(filename));
    }

    builder.libdir(&root_path.join("modules/lib"));
    builder.includedir(&root_path.join("modules"));
    for (submodule, _) in &module_toml.dependencies {
        builder.linklib(&submodule);
    }

    builder.output(&root_path.join(&module_toml.project.name));
    builder.exec();
}

pub fn build_submodule(root_path: &PathBuf, submodule_path: &str) {
    let mut builder = Build::new();
    builder.flag("-c");

    let (module_path, output_path) = (
        root_path.join("modules").join(&submodule_path),
        root_path.join("modules/lib"),
    );

    let module_toml = std::fs::read_to_string(module_path.join("module.toml"))
        .expect("Failed to read module.toml");
    let module: Module = toml::from_str(&module_toml).expect("Failed to parse module.toml");

    for (submodule, _) in &module.dependencies {
        build_submodule(root_path, submodule);
    }

    for filename in module.sources.source {
        builder.source(&module_path.join(filename));
    }

    builder.libdir(&output_path);
    builder.includedir(&module_path);
    for (submodule, _) in module.dependencies {
        builder.linklib(&submodule);
    }

    builder.output(&output_path.join(&module.project.name));
    builder.exec();
}
