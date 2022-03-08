use super::{super::errors::{BadFileType, Signal}, show::format_value, reader::{read_code, read_script, read_archive}};
use hyperbuild_core::hypscript::{
    exec::Executor,
    loader::LocalModuleLoader,
    env::{CallResult, IObject},
};

use std::{path::PathBuf, io::{Result, Error, ErrorKind}, ffi::OsStr};
use hyperbuild_core::hypscript::env::IClass;

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};

use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

fn caller() {
    struct Localizations;

    let loader: FluentLanguageLoader = fluent_language_loader!();
    loader
        .load_languages(&Localizations, &[loader.fallback_language()])
        .unwrap();

    assert_eq!(
        "Hello \u{2068}Bob 23\u{2069}!",
        // Compile time check for message id, and the `name` argument,
        // to ensure it matches what is specified in the `fallback_language`'s
        // fluent resource file.
        fl!(loader, "hello-arg", name = format!("Bob {}", 23))
    );

    fl!(
        loader: FluentLanguageLoader,
        "message_id",
        arg1 = 11,
        arg2 = "value",
        arg3 = "",
    );

    println!("Ici: {}", fl!(
        loader: FluentLanguageLoader,
        "message_2",
        arg_a = 12,
        arg_b = "value",
        arg_c = "",
    ));
}

macro_rules! mymac {
    (a: $x:expr) => {fl!(loader, "other")}
    (a: $x:expr) => {fl!(loader, "msg", argx = "", sup = 3, nono=23)}
    (a: $x:expr) => {tr!("One thing" | "{n} things" % $x)}
    (b: $x:expr) => {format!("R:{}", tr!("One field" | "{n} fields" % $x))}
    (c: $x:expr, d: $y:expr) => {format!("R:{}", tr!("X" => "One field {}" | "{n} fields" % $x, "Z", $y))}
}

pub(super) fn run_file(input: PathBuf, dirs: Vec<PathBuf>, show_value: bool) -> Result<()> {
    let mut loader = LocalModuleLoader::new(dirs)?;
    let code = if let Some(ext) = input.extension().and_then(OsStr::to_str) {
        match ext {
            "hbc" => read_code(input)?,
            "hbs" => read_script(input)?,
            "hba" => read_archive(input, &mut loader)?,
            _ => { return Err(BadFileType::new(input).into()); }
        }
    } else {
        return Err(BadFileType::new(input).into());
    };

    let mut executor = Executor::with_importer(Box::new(loader));

    match executor.execute(code) {
        CallResult::Signal(signal) => Err(Error::new(ErrorKind::Interrupted, Signal::new(signal))),
        CallResult::Null => {
            if show_value {
                println!("{}: <Null>", tr!("Returned value"));
            }

            Ok(())
        }
        CallResult::Value(res) => {
            if show_value {
                match format_value(executor.get_scope(), res.clone()) {
                    Err(signal) => Err(Error::new(ErrorKind::Interrupted, Signal::new(signal))),
                    Ok(v) => {
                        println!("{} [{}]: {}", tr!("Returned value"), res.get_class().get_name(), v);

                        Ok(())
                    }
                }
            } else {
                Ok(())
            }
        }
    }
}
