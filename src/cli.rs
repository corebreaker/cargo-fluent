use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[clap(version, author, about, subcommand_required = true, arg_required_else_help = true, propagate_version = true)]
pub struct CliArgs {
    /// Quiet mode, don't show any output, only errors
    #[clap(short, long, global = true)]
    pub quiet: bool,

    /// Destination directory, overrides the value in config files
    #[clap(short, long, global = true, value_name = "DIR")]
    pub output: Option<String>,

    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Scan source files (rust files only, `*.rs`) and create or update the FLT files
    #[clap(after_help = "\
        This command will read config files (Cargo.toml and i18n.toml) from the current directory. \
        Then, it will scan all source files and all translation messages will be extracted from the sources. \
        Finally, the Fluent translation file for the default language will be created. \
        If some Fluent files exist, they could be updated.\n\
        \n\
        The destination directory contains all Fluent files which will be written.\n\
        The domain will be used to build the name of Fluent files. \
        The domain is got from the name of the current crate, but it can be overridden with the command argument.\n\
        The argument `--convert-macros` will just convert macros. No dependency or instructions `use` will be added.\
    ")]
    Scan(ScanArgs),

    /// Scan PO files and create or update the FLT files
    #[clap(after_help = "\
        This command will read PO files and from the config files read from the current directory, \
        the Fluent translation file for the default language will be created. \
        If some Fluent files exist, they could be updated.\n\
        \n\
        The destination directory contains all Fluent files which will be written.\n\
        The domain will be used to build the name of Fluent files. \
        The domain is got from the name of the current crate, but it can be overridden with the command argument\n\
        If some paths are specified in command line, \
        there will be the PO files read instead those found by default from values taken in config files. \
        The list of paths can be either files or directories.\
    ")]
    Convert(ConvertArgs),

    /// Start a GUI for editing the FLT files
    #[clap(after_help = "\
        An editor will opened and the existing Fluent files found from the current directory could be modified. \
        The editor could create Fluent files, that add a new translation in the project. \
        Also, an existing file could be deleted.\
    ")]
    Edit,
}

#[derive(Args)]
pub struct ScanArgs {
    /// Domain, overrides the name of the current crate
    #[clap(short, long)]
    pub domain: Option<String>,

    /// Convert call of macro `tr!` by call of `fl!`
    #[clap(short, long)]
    pub convert_macros: bool,
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Domain, overrides the name of the current crate
    #[clap(short, long)]
    pub domain: Option<String>,

    /// PO files or directories used instead files found with config files
    #[clap(value_name = "PATH")]
    pub paths: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        CliCommand::command().debug_assert()
    }
}
/*
let args = clap_app!(CargoFluent =>
    (version: crate_version!())
    (author: crate_authors!())
    (about: crate_description!())
    (@setting SubcommandRequiredElseHelp)
    (@arg quiet: -q --quiet !required +global "Quiet mode, don't show any output, only errors")
    (@arg output: -o --output !required +global "Destination directory, overrides the value in config files")
    (@subcommand scan =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Scan source files (rust files only) and create or update the FLT files")
        (after_help: "\
            This command will read config files (Cargo.toml and i18n.toml) from the current directory. \
            Then, it will scan all source files and all translation messages will be extracted from the sources. \
            Finally, the Fluent translation file for the default language will be created. \
            If some Fluent files exist, they could be updated.\n\
            \n\
            The destination directory contains all Fluent files which will be written.\n\
            The domain will be used to build the name of Fluent files. \
            The domain is got from the name of the current crate, \
            but it can be overridden with the command argument.\n\
            The argument `--convert-macros` will just convert macros. \
            No dependency or instructions `use` will be added.\
        ")
        (@arg domain: -d --domain !required "Domain, overrides the name of the current crate")
        (@arg convert_macros: -c --convert-macros !required "Convert call of macro `tr!` by call of `fl!`")
    )
    (@subcommand convert =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Scan PO files and create or update the FLT files")
        (after_help: "\
            This command will read PO files and from the config files read from the current directory, \
            the Fluent translation file for the default language will be created. \
            If some Fluent files exist, they could be updated.\n\
            \n\
            The destination directory contains all Fluent files which will be written.\n\
            The domain will be used to build the name of Fluent files. \
            The domain is got from the name of the current crate, \
            but it can be overridden with the command argument\n\
            If some paths are specified in command line, \
            there will be the PO files read instead those found by default from values taken in config files. \
            The list of paths can be either files or directories.\
        ")
        (@arg domain: -d --domain !required "Domain, overrides the name of the current crate")
        (@arg PATHS: ... !required "PO files or directories used instead files found with config files")
    )
    (@subcommand edit =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Start a GUI for editing the FLT files")
        (after_help: "\
            An editor will opened and the existing Fluent files found \
            from the current directory could be modified. \
            The editor could create Fluent files, that add a new translation in the project. \
            Also, an existing file could be deleted.\
        ")
    )
).get_matches();
*/
