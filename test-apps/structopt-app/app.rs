use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct AppArgs {
    #[structopt(short = "h", long = "help")]
    help: bool,

    #[structopt(long = "number")]
    number: u32,

    #[structopt(long = "opt-number")]
    opt_number: Option<u32>,

    #[structopt(long = "width", default_value = "10", parse(try_from_str = parse_width))]
    width: u32,

    #[structopt(name = "FILE", parse(from_os_str))]
    free: Vec<std::path::PathBuf>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    let w = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    let args = AppArgs::from_args();
    println!("{:#?}", args);
}
