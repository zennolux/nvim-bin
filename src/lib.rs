pub mod configuration {
    use chrono::Local;
    use core::panic;
    use std::{env, error::Error, process::Command};

    pub enum Kind {
        PluginManager,
        NvimConfig,
    }

    struct Configuration {
        git_repo: String,
        save_path: String,
        bakup_path: String,
    }

    impl Configuration {
        fn new(git_repo: String, save_path: String) -> Self {
            let time = format!("{}", Local::now().format("%y%m%d%H%M%S"));
            let save_path = save_path.replace("~", &env::var("HOME").unwrap());
            let bakup_path = format!("{}_bakup{}", save_path, time);
            Self {
                git_repo,
                save_path,
                bakup_path,
            }
        }

        fn backup(&self) -> Result<(), Box<dyn Error>> {
            Command::new("mv")
                .arg(&self.save_path)
                .arg(&self.bakup_path)
                .output()?;
            Ok(())
        }
    }

    pub fn start() {
        println!("Started...");
        let installed_plugin_manager = self::install(Kind::PluginManager).unwrap();
        let installed_nvim_config = self::install(Kind::NvimConfig).unwrap();
        println!(
            "{:?}\n{:?}\nEnjoy your neovim journey now!",
            installed_plugin_manager, installed_nvim_config
        );
    }

    pub fn install(kind: Kind) -> Result<String, Box<dyn Error>> {
        match kind {
            Kind::PluginManager => self::install_plugin_manager(Configuration::new(
                String::from("https://github.com/folke/lazy.nvim.git"),
                String::from("~/.local/share/nvim/lazy/lazy.nvim"),
            )),
            Kind::NvimConfig => self::install_nvim_config(Configuration::new(
                String::from("https://github.com/zennolux/nvim.git"),
                String::from("~/.config/nvim"),
            )),
        }
    }

    fn install_plugin_manager(conf: Configuration) -> Result<String, Box<dyn Error>> {
        conf.backup()?;

        match Command::new("git")
            .arg("clone")
            .arg("--filter=blob:none")
            .arg(conf.git_repo)
            .arg("--branch=stable")
            .arg(conf.save_path)
            .output()
        {
            Ok(res) => {
                if res.status.success() {
                    return Ok(
                        "Successfully installed nvim plugin manager (lazy.nvim).".to_string()
                    );
                }
                let stderr = String::from_utf8_lossy(&res.stderr).trim().to_owned();
                panic!("{:?}", stderr);
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    fn install_nvim_config(conf: Configuration) -> Result<String, Box<dyn Error>> {
        conf.backup()?;

        match Command::new("git")
            .arg("clone")
            .arg(conf.git_repo)
            .arg(&conf.save_path)
            .output()
        {
            Ok(res) => {
                if res.status.success() {
                    return Ok("Successfully configured nvim.".to_string());
                }
                let stderr = String::from_utf8_lossy(&res.stderr).trim().to_owned();
                panic!("{:?}", stderr);
            }
            Err(error) => Err(Box::new(error)),
        }
    }
}
