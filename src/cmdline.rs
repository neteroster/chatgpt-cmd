use core::fmt;
use std::path::{Path, PathBuf};

pub struct CmdChat {
    pub chat_content: String,
}

impl Into<String> for CmdChat {
    fn into(self) -> String {
        self.chat_content
    }
}

pub enum CmdOperation {
    ClearContext,
    SaveContext(PathBuf),
    ReadContext(PathBuf),
    QuitCmd,
}

pub enum CmdLine {
    Chat(CmdChat),
    Operation(CmdOperation),
}

impl From<CmdChat> for CmdLine {
    fn from(value: CmdChat) -> Self {
        Self::Chat(value)
    }
}

impl From<CmdOperation> for CmdLine {
    fn from(value: CmdOperation) -> Self {
        Self::Operation(value)
    }
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

impl<S> TryFrom<TryFromWrapper<S>> for CmdOperation
where
    S: Into<String>,
{
    type Error = CmdParseError;
    fn try_from(s: TryFromWrapper<S>) -> Result<Self, Self::Error> {
        let s: String = s.extract().into();
        let mut cmd = s.split(' ').into_iter();

        match cmd.next().ok_or(CmdParseError)? {
            "!save" => Ok(Self::SaveContext(cmd.next().ok_or(CmdParseError)?.into())),
            "!read" => Ok(Self::ReadContext(cmd.next().ok_or(CmdParseError)?.into())),
            "!clear" => Ok(Self::ClearContext),
            "!quit" => Ok(Self::QuitCmd),
            _ => Err(CmdParseError),
        }
    }
}

impl<S> TryFrom<TryFromWrapper<S>> for CmdLine
where
    S: Into<String>,
{
    type Error = CmdParseError;
    fn try_from(s: TryFromWrapper<S>) -> Result<Self, Self::Error> {
        let s: String = s.extract().into();
        if s.starts_with('!') {
            CmdOperation::try_from(TryFromWrapper(s)).map(|op| op.into())
        } else {
            Ok(CmdChat { chat_content: s }.into())
        }
    }
}
