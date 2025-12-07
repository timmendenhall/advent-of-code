pub struct Config {
    pub file_path: String,
    pub strategy: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let strategy = args[2].clone();

        Ok(Config {
            file_path,
            strategy,
        })
    }
}
