use super::path_pattern::parse_path_pattern;
use clap::{Parser, Args, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(version, author, about, subcommand_required = true, arg_required_else_help = true, propagate_version = true)]
#[clap(after_help = "All subcommands must be executed in a crate or a workspace")]
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
        The argument `--convert-macros` will just convert macros. No dependency or instructions `use` will be added.\n\
        Only source files (`*.rs`) in the directory `src` of the current crate will be scanned. \
        Neither files in the directory `test` immediately inside the current crate, \
        nor the file `build.rs` will be scanned.\
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
        there will be the PO files read instead those found by default from values taken in config files.\n\
        The list of paths can be either files or directories.\n\
        If paths contain wildcards or specify directories, only PO files will be selected, \
        files with an extension `.po`.\n\
        Path must be relatives to the current directory.\n\
        \n\
        The domain will be the name of the Fluent file (FLT file).\n\
        Therefore, the name of each PO file found will be the domain.\n\
        \n\
        The flag `merge` will merge all translations in PO files for all found domains into one Fluent file. \
        A merge is done for each found language.\n\
        If it was not specified, \
        the list of paths overrides the argument `domain` and the domain found in config files.\
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
    pub(crate) domain: Option<String>,

    /// Convert call of macro `tr!` by call of `fl!`
    #[clap(short, long)]
    pub(crate) convert_macros: bool,
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Domain, overrides the name of the current crate
    #[clap(short, long)]
    pub(crate) domain: Option<String>,

    /// Merge all PO files for all found domains into one FLT file, one merge per found language
    #[clap(short, long)]
    pub(crate) merge: bool,

    /// Include fuzzy entries from PO files
    #[clap(short = 'f', long)]
    pub(crate) include_fuzzy: bool,

    /// PO files or directories used instead files found with config files
    #[clap(value_name = "PATH", parse(try_from_str = parse_path_pattern))]
    po_files: Vec<Vec<PathBuf>>,
}

impl ConvertArgs {
    pub(crate) fn po_files(&self) -> Option<Vec<&Path>> {
        if self.po_files.is_empty() {
            None
        } else {
            Some(self.po_files.iter().flatten().map(PathBuf::as_path).collect())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        CliArgs::command().debug_assert()
    }
}
