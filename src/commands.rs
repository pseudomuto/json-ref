use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// An enum representing the available commands
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    /// Prints the document with all references resolved
    Resolve,
    /// Validates that all references in the document are resolveable
    Validate,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "resolve" => Ok(Command::Resolve),
            "validate" => Ok(Command::Validate),
            _ => Err("not a valid command"),
        }
    }
}

impl Command {
    /// Executes the command with the given arguments
    pub fn execute(self, args: HashMap<&str, &str>) {
        if let Some(uri) = args.get("URI") {
            match self {
                Command::Resolve => resolve(uri, args.contains_key("yaml")),
                Command::Validate => validate(uri),
            }
        }
    }
}

fn resolve(uri: &str, is_yaml: bool) {
    // TODO: Do something valuable
    println!("Running resolve with uri: {}, yaml: {}", uri, is_yaml);
}

fn validate(uri: &str) {
    // TODO: Do something valuable
    println!("Running validate with uri: {}", uri);
}

#[cfg(test)]
mod test {
    use super::Command;

    #[test]
    fn debug_and_display_output() {
        assert_eq!("Found Resolve", format!("Found {:?}", Command::Resolve));
        assert_eq!("Found Validate", format!("Found {:?}", Command::Validate));

        assert_eq!("Found Resolve", format!("Found {}", Command::Resolve));
        assert_eq!("Found Validate", format!("Found {}", Command::Validate));
    }

    #[test]
    fn from_string() {
        assert_eq!(Ok(Command::Resolve), "resolve".parse());
        assert_eq!(Ok(Command::Resolve), "RESOLVE".parse());

        assert_eq!(Ok(Command::Validate), "validate".parse());
        assert_eq!(Ok(Command::Validate), "VALIDATE".parse());
    }
}
