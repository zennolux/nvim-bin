pub mod configuration {
    use chrono::Local;
    use core::panic;
    use std::{env, error::Error, process::Command};

    pub fn start() {
        println!("Started...");
        let installed = self::install_plugin_manager().unwrap();
        let cloned = self::clone_nvim_config().unwrap();
        println!(
            "{:?}\n{:?}\nEnjoy your neovim journey now!",
            installed, cloned
        );
    }

    fn install_plugin_manager() -> Result<String, Box<dyn Error>> {
        let repo = "https://github.com/folke/lazy.nvim.git";
        let path = format!(
            "{}/.local/share/nvim/lazy/lazy.nvim",
            env::var("HOME").unwrap()
        );
        let time = format!("{}", Local::now().format("%y%m%d%H%M%S"));

        Command::new("mv")
            .arg(&path)
            .arg(format!("{}.bakup_{}", path, time))
            .output()?;

        match Command::new("git")
            .arg("clone")
            .arg("--filter=blob:none")
            .arg(repo)
            .arg("--branch=stable")
            .arg(path)
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

    fn clone_nvim_config() -> Result<String, Box<dyn Error>> {
        let repo = "https://github.com/zennolux/nvim.git";
        let path = format!("{}/.config/nvim", env::var("HOME").unwrap());
        let time = format!("{}", Local::now().format("%y%m%d%H%M%S"));

        Command::new("mv")
            .arg(&path)
            .arg(format!("{}.bakup_{}", path, time))
            .output()?;

        match Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(&path)
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
