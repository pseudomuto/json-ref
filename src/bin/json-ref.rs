extern crate clap;
extern crate json_ref;

use std::collections::HashMap;

use clap::{App, AppSettings, ArgMatches, SubCommand};
use json_ref::Command;

fn main() {
    let matches = App::new("json-ref")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("resolve")
                .about("Prints the document with all references resolved")
                .args_from_usage(
                    "<URI> 'The URI to parse. Can be a file or URL.'
                    -y, --yaml 'Output result in YAML",
                ),
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validates that all references in the document are resolveable")
                .arg_from_usage("<URI> 'The URI to validate. Can be a file or URL.'"),
        )
        .get_matches();

    if let (cmd, Some(matches)) = matches.subcommand() {
        if let Ok(c) = cmd.parse::<Command>() {
            c.execute(args_for_command(c, &matches));
        }
    }
}

fn args_for_command<'a>(cmd: Command, matches: &'a ArgMatches) -> HashMap<&'a str, &'a str> {
    let mut args = HashMap::new();
    args.insert("URI", matches.value_of("URI").unwrap());

    if cmd == Command::Resolve {
        if matches.is_present("yaml") {
            args.insert("yaml", "true");
        }
    }

    args
}
