use chrono::Local;
use std::{env, process::Command};

fn main() {
    install_nvim_plugin_manager();
    clone_my_nvim_config();
    println!("Enjoy your neovim journey now!");
}

fn install_nvim_plugin_manager() {
    let repo = "https://github.com/folke/lazy.nvim.git";
    let path = format!(
        "{}/.local/share/nvim/lazy/lazy.nvim",
        env::var("HOME").unwrap()
    );
    let time = format!("{}", Local::now().format("%y%m%d%H%M%S"));

    Command::new("mv")
        .arg(&path)
        .arg(format!("{}.bakup_{}", path, time))
        .output()
        .expect("Failed to backup the old one");

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
                println!("Successfully installed nvim plugin manager (lazy.nvim).");
                return ();
            }
            let stderr = String::from_utf8_lossy(&res.stderr).trim().to_owned();
            println!("{:?}", stderr);
        }
        Err(error) => println!("{:?}", error),
    }
}

fn clone_my_nvim_config() {
    let repo = "https://github.com/zennolux/nvim.git";
    let path = format!("{}/.config/nvim", env::var("HOME").unwrap());
    let time = format!("{}", Local::now().format("%y%m%d%H%M%S"));

    Command::new("mv")
        .arg(&path)
        .arg(format!("{}.bakup_{}", path, time))
        .output()
        .expect("Failed to backup the old one");

    match Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(&path)
        .output()
    {
        Ok(res) => {
            if res.status.success() {
                println!("Successfully configured nvim.");
                return ();
            }
            let stderr = String::from_utf8_lossy(&res.stderr).trim().to_owned();
            println!("{:?}", stderr);
        }
        Err(error) => println!("{:?}", error),
    }
}
