use core::fmt;
use std::path::Path;

pub struct CmdChat {
    pub chat_content: String,
}

pub enum CmdOperationType {
    ClearContext,
    SaveContext(String),
    ReadContext(String),
    QuitCmd,
}

pub struct TryFromWrapper<T>(pub T);

impl<T> From<T> for TryFromWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> TryFromWrapper<T> {
    pub fn extract(self) -> T {
        self.0
    }
}

#[derive(Debug)]
pub struct CmdParseError;

impl fmt::Display for CmdParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cmd parse error.")
    }
}

impl std::error::Error for CmdParseError {}

impl<S> TryFrom<TryFromWrapper<S>> for CmdOperationType
where
    S: Into<String>,
{
    type Error = CmdParseError;
    fn try_from(s: TryFromWrapper<S>) -> Result<Self, Self::Error> {
        let s: String = s.extract().into();
        let mut cmd = s.split(' ').into_iter();

        match cmd.next().ok_or(CmdParseError)? {
            "save" => Ok(Self::SaveContext(
                cmd.next().ok_or(CmdParseError)?.to_string(),
            )),
            "read" => Ok(Self::ReadContext(
                cmd.next().ok_or(CmdParseError)?.to_string(),
            )),
            "clear" => Ok(Self::ClearContext),
            "quit" => Ok(Self::QuitCmd),
            _ => Err(CmdParseError),
        }
    }
}
