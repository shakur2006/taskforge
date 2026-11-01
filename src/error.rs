use std::io;
use std::fmt;
use std::io::Error;

// ----- Task Error Enum ----- //
#[derive(Debug)]
pub enum TaskError{
    Io(io::Error),
    Serialization(serde_json::Error),
    TaskNotFound(u32),
    InvalidInput(String),
}

// ----- fmt implemttaion blockfor task error -----//
impl fmt::Display for TaskError{
    fn fmt(&self, f:  &mut fmt::Formatter<'_>)-> fmt::Result{
        match self{
            TaskError::Io(err) => write!(f, "File error: {}", err),
            TaskError::Serialization(err) => write!(f, "Data serilaization error: {}", err),
            TaskError::TaskNotFound(id) => write!(f, "Task with the ID {} not found", id),
            TaskError::InvalidInput(message) => write!(f, "Invald input: {}", message)

        }
    }
}

impl std::error::Error for TaskError{}

impl From<io::Error> for TaskError{
    fn from(err: io::Error) -> Self{
        TaskError::Io(err)
    }
}

impl From<serde_json::Error> for TaskError{
    fn from(value: serde_json::Error) -> Self {
        TaskError::Serialization(value)
    }
}