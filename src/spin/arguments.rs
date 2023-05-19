use clap::{Parser, Subcommand};
use crate::echo;

#[derive(Parser)]
#[command(name = "spin")]
#[command(bin_name = "spin")]
#[command(about = "A CLI string generation tool", long_about = None)]
pub struct Arguments {
    /// Surround the output with double quotes
    #[arg(short, long, conflicts_with = "single_quotes", )]
    pub double_quotes: bool,

    /// Surround the output with single quotes
    #[arg(short, long, conflicts_with = "double_quotes")]
    pub single_quotes: bool,

    /// Repeat the generation
    #[arg(short, long, value_name = "COUNT", default_value_t = 1)]
    pub repeat: usize,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Echo the input text with any post processing applied
    #[command(arg_required_else_help = true)]
    Echo(echo::EchoArguments),
    /// Generate a random UUID
    Uuid,
}