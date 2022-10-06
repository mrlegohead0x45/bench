use clap::{ArgGroup, Parser};
use std::time;

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Parser, Debug)]
#[command(author, version)]
#[clap(group(ArgGroup::new("count").required(true).args(&["num", "time"])))]
struct Args {
    /// Format to output results in
    #[arg(value_enum, short, long, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Command to time the execution of
    cmd: String,

    /// Verbosity
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// How many times to run the command
    #[arg(short, long, group = "count")]
    num: Option<usize>,

    /// How long to run for
    #[arg(short, long, group = "count")]
    time: Option<String>,

    /// Extra options to pass to CMD, preceded by --.
    /// E.g `bench -n 3 ls -- -l -a`
    cmd_options: Vec<String>,
}

fn main() {
    let args = Args::parse();
    // let format = match args.format {
    //     Some(f) => f,
    //     None => OutputType::Text
    // };
    println!("{:?}", args);
}
