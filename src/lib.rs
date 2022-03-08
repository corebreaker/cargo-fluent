mod files;
pub mod config;
pub mod commands;

pub fn show() -> std::io::Result<()> {
    let filename = match std::env::args_os().skip(1).next() {
        Some(name) => name,
        None => {
            let error = simple_error::SimpleError::new("No file specified");

            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
        }
    };

    let path = std::path::PathBuf::from(filename);
    let src = files::source::RustSource::read(&path)?;

    for t in src.translations {
        println!("%> {:?}", t);
    }

    Ok(())
}