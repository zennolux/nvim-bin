use nvim::configuration as nvimconf;

fn main() {
    println!("Started...");
    let installed_plugin_manager = nvimconf::install(nvimconf::Kind::PluginManager).unwrap();
    let installed_nvim_config = nvimconf::install(nvimconf::Kind::NvimConfig).unwrap();
    println!(
        "{}\n{}\nEnjoy your neovim journey now!",
        installed_plugin_manager, installed_nvim_config
    );
}
