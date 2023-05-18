use clap::{Parser, Subcommand};
use crate::echo;

#[derive(Parser)]
#[command(name = "spin")]
#[command(bin_name = "spin")]
#[command(about = "A CLI string generation tool", long_about = None)]
pub struct Arguments {
    /// Surround the output with double quotes
    #[arg(short = 'd', long, conflicts_with = "single_quotes", )]
    pub double_quotes: bool,
    /// Surround the output with double quotes
    #[arg(short = 's', long, conflicts_with = "double_quotes")]
    pub single_quotes: bool,
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