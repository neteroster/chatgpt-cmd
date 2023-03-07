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

impl<S> From<S> for CmdOperationType
where
    S: Into<String>,
{
    
}
