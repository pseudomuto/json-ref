extern crate clap;
extern crate json_ref;

use clap::{App, AppSettings, SubCommand};

fn main() {
    App::new("json-ref")
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
}
