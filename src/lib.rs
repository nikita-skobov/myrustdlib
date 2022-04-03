use std::io;


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

pub type Sresult<T> = Result<T, Serr>;

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
