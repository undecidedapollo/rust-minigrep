use std::error::Error;
use std::env;
use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let query = args.get(1).ok_or("Missing query parameter")?;
    let file_name = args.get(2);

    let cfg = match file_name {
        Some(file_name) => Config::from_file(
            args.get(1).ok_or("Missing query parameter")?, 
            file_name,
            ignore_case,
        ),
        None => Config::from_stream(query, ignore_case),
    };

    minigrep::run(&cfg)
}
