mod arguments;
pub use crate::arguments::*;

mod echo;
pub use crate::echo::EchoArguments;
mod uuid;

mod post_process;

/// Generate the string using the given arguments
/// 
/// # Arguments
/// 
/// * `arguments` - A struct of arguments for spin including commands and input
/// 
/// # Examples
/// 
/// ```
/// use spin::*;
/// let arguments = Arguments {
///     double_quotes: true,
///     single_quotes: false,
///     command: Commands::Echo(EchoArguments {
///         input: String::from("example")
///     })
/// };
/// 
/// let output = spin::generate(&arguments);
/// ```
pub fn generate(arguments: &Arguments) -> String {
    let output = match &arguments.command {
        Commands::Echo(echo_arguments)=> echo::echo(echo_arguments),
        Commands::Uuid => uuid::uuid()
    };

    post_process::post_process(output, arguments)
}