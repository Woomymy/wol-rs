use libwol_rs::MacParseError;

#[derive(Debug)]
pub struct Error(String);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<MacParseError> for Error {
    fn from(e: MacParseError) -> Self {
        match e {
            MacParseError::InvalidInput => Self(format!("Invalid mac address!")),
            MacParseError::InvalidLenght => Self(format!("Invalid mac address length!")),
        }
    }
}
