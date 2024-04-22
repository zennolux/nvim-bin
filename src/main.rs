use nvimconf::configuration;

fn main() {
    println!("Started...");
    let installed_plugin_manager =
        configuration::install(configuration::Kind::PluginManager).unwrap();
    let installed_nvim_config = configuration::install(configuration::Kind::NvimConfig).unwrap();
    println!(
        "{}\n{}\nEnjoy your neovim journey now!",
        installed_plugin_manager, installed_nvim_config
    );
}
