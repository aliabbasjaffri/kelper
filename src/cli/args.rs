use crate::cli::formats::LogFormat;
use crate::cli::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable verbose logging. Use multiple v's for increased verbosity:
    /// -v: WARN level
    /// -vv: INFO level
    /// -vvv: DEBUG level
    /// -vvvv: TRACE level
    #[arg(short = 'v', long = "verbose", global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Log format to use (default: plain for local development)
    #[arg(
        long = "log-format",
        default_value = "plain",
        global = true,
        requires = "verbose"
    )]
    pub log_format: LogFormat,

    #[command(subcommand)]
    pub command: Commands,
}
