use tonic::Status;

pub(crate) enum Error {
    IDGenerationError(libxid::IDGenerationError),
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::IDGenerationError(e) => Status::internal(["failed to generate xid: ", e.to_string().as_str()].concat())
        }
    }
}