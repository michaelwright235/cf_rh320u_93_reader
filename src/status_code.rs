use thiserror::Error;

/// There're two types of possible errors:
/// `UsbError` and `CommandError`. If the command is not implemented, a
/// `NotImplemented` variant is returned.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum ReaderError {
    /// UsbError is returned if there was some kind of connection problem.
    #[error("{0}")]
    UsbError(#[from] rusb::Error),

    /// CommandError is returned if a reader couldn't proccess given command in some way.
    #[error("{0}")]
    CommandError(#[from] StatusCode),

    /// This command is not implemented yet.
    #[error("Not implemented yet")]
    NotImplemented,
}

/// Status codes that a reader can throw.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum StatusCode {
    #[error("Success")]
    Ok,

    #[error("Failure")]
    Failure,

    #[error("Timeout error")]
    TimeoutError,

    #[error("Card doesn't exist")]
    NoCard,

    #[error("Receiving card data error")]
    InvalidData,

    #[error("The input parameter or input command format is incorrect")]
    AuthenticationFailure,

    #[error("Internal error")]
    InternalError,

    #[error("Unknown error")]
    UnknownError,

    #[error("Operation error")]
    OperationError,

    #[error("Unknown command")]
    UnknownCommand,

    #[error("Unsupported command for this type of card")]
    UnsupportedCommandForCard,

    #[error("Invalid command format")]
    InvalidCommandFormat,

    #[error("In the FLAG parameter of the command, OPTION mode is not supported (only for 15693 commands)")]
    UnsupportedOptionForm, // ?

    #[error("Specified block doesn't exit")]
    BlockDoesntExist,

    #[error("Specified block is locked")]
    BlockIsLocked,

    #[error("Couldn't lock specified block")]
    LockBlockFailure,

    #[error("Failed to write data")]
    WriteFailure,

    #[error("Received unknown status code: {0}")]
    UnknownCode(u8),
}

impl From<u8> for StatusCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x80 => Self::Ok,
            0x81 => Self::Failure,
            0x82 => Self::TimeoutError,
            0x83 => Self::NoCard,
            0x84 => Self::InvalidData,
            0x85 => Self::AuthenticationFailure,
            0x86 => Self::InternalError,
            0x87 => Self::UnknownError,
            0x89 => Self::OperationError,
            0x8f => Self::UnknownCommand,
            0x90 => Self::UnsupportedCommandForCard,
            0x91 => Self::InvalidCommandFormat,
            0x92 => Self::UnsupportedOptionForm,
            0x93 => Self::BlockDoesntExist,
            0x94 => Self::BlockIsLocked,
            0x95 => Self::LockBlockFailure,
            0x96 => Self::WriteFailure,
            _ => Self::UnknownCode(byte),
        }
    }
}

impl From<StatusCode> for u8 {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode::Ok => 0x80,
            StatusCode::Failure => 0x81,
            StatusCode::TimeoutError => 0x82,
            StatusCode::NoCard => 0x83,
            StatusCode::InvalidData => 0x84,
            StatusCode::AuthenticationFailure => 0x85,
            StatusCode::InternalError => 0x86,
            StatusCode::UnknownError => 0x87,
            StatusCode::OperationError => 0x89,
            StatusCode::UnknownCommand => 0x8f,
            StatusCode::UnsupportedCommandForCard => 0x90,
            StatusCode::InvalidCommandFormat => 0x91,
            StatusCode::UnsupportedOptionForm => 0x92,
            StatusCode::BlockDoesntExist => 0x93,
            StatusCode::BlockIsLocked => 0x94,
            StatusCode::LockBlockFailure => 0x95,
            StatusCode::WriteFailure => 0x96,
            StatusCode::UnknownCode(byte) => byte,
        }
    }
}
