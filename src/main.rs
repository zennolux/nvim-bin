use nvim::configuration;

fn main() {
    match configuration::start() {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
    println!("Enjoy your neovim journey now!");
}
