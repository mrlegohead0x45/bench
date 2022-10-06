use clap::{ArgGroup, Parser};

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Parser, Debug)]
#[command(author, version)]
#[clap(group(ArgGroup::new("count").required(true).args(&["num", "time"])))]
pub struct Args {
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

    /// How long to run for,
    /// in the format [[hh]:mm:]ss
    #[arg(short, long, group = "count")]
    time: Option<String>,

    /// Extra options to pass to CMD, preceded by --.
    /// E.g `bench -n 3 ls -- -l -a`
    cmd_options: Vec<String>,
}
