pub fn echo(arguments: &EchoArguments) -> String {
    arguments.input.clone()
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct EchoArguments {
    pub input: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echoes_input() {
        let arguments = EchoArguments {
            input: String::from("test"),
        };
        assert!(echo(&arguments) == String::from("test"));
    }
}
