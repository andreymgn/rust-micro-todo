use tonic::Status;

#[derive(Debug)]
pub enum Error {
    NotFound,
    IDGenerationError,
    AlreadyCompleted,
    SQLError(sqlx::Error),
}

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self::not_found("todo not found"),
            Error::IDGenerationError => Self::internal("failed to generate id"),
            Error::AlreadyCompleted => Self::invalid_argument("todo already completed"),
            Error::SQLError(err) => match err {
                sqlx::error::Error::Configuration(e) => Self::internal(e.to_string()),
                sqlx::error::Error::Database(e) => Self::internal(e.to_string()),
                sqlx::error::Error::Io(e) => Self::internal(e.to_string()),
                sqlx::error::Error::Tls(e) => Self::internal(e.to_string()),
                sqlx::error::Error::Protocol(e) => Self::internal(e.to_string()),
                sqlx::error::Error::RowNotFound => Self::not_found("todo not found"),
                sqlx::error::Error::ColumnIndexOutOfBounds { .. } => {
                    Self::internal("ColumnIndexOutOfBounds")
                }
                sqlx::error::Error::ColumnNotFound(e) => Self::internal(e.to_string()),
                sqlx::error::Error::ColumnDecode { .. } => Self::internal("ColumnDecode"),
                sqlx::error::Error::Decode(e) => Self::internal(e.to_string()),
                sqlx::error::Error::PoolTimedOut => Self::internal("PoolTimedOut"),
                sqlx::error::Error::PoolClosed => Self::internal("PoolClosed"),
                sqlx::error::Error::WorkerCrashed => Self::internal("WorkerCrashed"),
                sqlx::error::Error::Migrate(e) => Self::internal(e.to_string()),
                _ => Self::internal(format!("unknown sqlx error")),
            },
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::SQLError(err)
    }
}

impl From<libxid::IDGenerationError> for Error {
    fn from(_: libxid::IDGenerationError) -> Self {
        Self::IDGenerationError
    }
}
