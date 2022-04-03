use std::{io, fmt::Display};


/// String + error = Serr
/// Rust doesn't let you define implementations on things
/// that aren't defined in your library, so by wrapping a
/// string in a struct, we can define out own impl and conversions.
/// for example, we implement `From<io::Error> for Serr`
pub struct Serr {
    pub err: String,
}

impl From<io::Error> for Serr {
    fn from(src: io::Error) -> Self {
        Serr { err: src.to_string() }
    }
}
impl From<&str> for Serr {
    fn from(src: &str) -> Self {
        Serr { err: src.to_string() }
    }
}
impl Display for Serr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.err)
    }
}

pub type Sresult<T> = Result<T, Serr>;

/// a lot of times your program just
/// needs to get the first CLI argument,
/// or just error out...
pub fn first_cli_arg() -> Sresult<String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let first = args.first().ok_or("Missing first CLI argument")?;
    Ok(first.into())
}

#[macro_export]
macro_rules! main_or_exit {
    ($mainfn:tt) => {
        fn main() {
            if let Err(e) = $mainfn() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn into_serr_works() {
        let cb = || -> Sresult<()> {
            let _ = File::open("nonexistantfile.txt")?;
            Ok(())
        };
        assert!(cb().is_err());
    }
}
