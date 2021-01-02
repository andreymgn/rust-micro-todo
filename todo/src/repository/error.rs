use tonic::Status;

pub enum Error {
    NotFound,
    IDGenerationError,
    AlreadyCompleted,
}

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self::not_found("todo not found"),
            Error::IDGenerationError => Self::internal("failed to generate id"),
            Error::AlreadyCompleted => Self::invalid_argument("todo already completed"),
        }
    }
}
